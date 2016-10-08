'''
    A script for mapping opcode implementation functions to the opcode table
'''

import os
import re
import glob

# matches (instruction name)(opcode)(timing)(function name) from comments
# placed above functions.
FN_REGEX = re.compile(ur'//\s\'(?P<name>.*)\''
                      ur'\s(?P<code>CB [0-9A-F]{2}|[0-9A-F]{2})'
                      ur'\s(?P<time>\d+)'
                      ur'\s+pub fn (?P<fname>.+)\(', re.MULTILINE)

CODES = {}
CB_CODES = {}
MODULES = []

MODULE_IMPORT = u'use jeebie::instr::{0}::*;\n'

FILE_HEADER = (u'\nuse jeebie::core::cpu::CPU;\n'
               u'/// The type of functions that implement an opcode.\n'
               u'pub type OpcodeFunc = fn(&mut CPU) -> i32;\n\n'
               u'fn missing(cpu: &mut CPU) -> i32 {panic!("Opcode 0x{:02X} is '
               u'not implemented!", cpu.mem.read_b(cpu.reg.pc-1))}\n'
               u'fn missing_cb(cpu: &mut CPU) -> i32 {panic!("Opcode '
               u'0xCB{:02X} is not implemented!", '
               u'cpu.mem.read_b(cpu.reg.pc-1))}\n')

DISCLAIMER = (u'/// This file is autogenerated, '
              u'check the `generate_tables.py` source file.\n')

TABLE_DECL = u'\n\npub static {0}_TABLE : [OpcodeFunc; 256] = [\n'
DISASM_T_DECL = u'\n\npub static {0}_TABLE : [&\'static str; 256] = [\n'
TABLE_ROW = u'    {:>13}, {:>13}, {:>13}, {:>13},\n'
DISASM_T_ROW = u'    "{}", "{}", "{}", "{}",\n'

UNIMPLEMENTED = u'missing'
UNIMPLEMENTED_CB = u'missing_cb'


def process_opcode_file(file_name):
    '''
        Reads a file with opcode implementations.

        These are files that contain functions with a comment detailing the
        name of the instruction (used for disassembly), its opcode and
        timing values (expressed in machine cycles).

        The function adds this data to the CODES/CB_CODES variables.
    '''

    source_text = open(file_name, 'r').read()

    for match in FN_REGEX.finditer(source_text):
        group = match.groupdict()

        if group['code'].startswith('CB'):
            code = int(group['code'][-2:], 16)
            CB_CODES[code] = {
                'name': group['name'],
                'fn': group['fname'],
                'timing': group['time']
            }
        else:
            code = int(group['code'], 16)
            CODES[code] = {
                'name': group['name'],
                'fn': group['fname'],
                'timing': group['time']
            }

    module_name = os.path.splitext(os.path.basename(file_name))[0]

    if module_name not in ['mod', 'opcodes', 'timings']:
        MODULES.append(module_name)


def build_output():
    '''
        Generates the source file for mapping opcodes to instructions.

        The generated file contains two static arrays of function references
        that map the opcode (a byte value) to the function that implements it.

        Opcodes that have no mapping (either temporarily or are unmapped) will
        point to a `missing` function that will panic when called.

        Returns a list of strings that makes up the generated source file
    '''

    output = []
    output.append(DISCLAIMER)
    for module in MODULES:
        output.append(MODULE_IMPORT.format(module))

    output.append(FILE_HEADER)
    output.append(TABLE_DECL.format('OPCODE'))

    for i in xrange(0, 256, 4):
        if i % 16 == 0:
            output.append('    // ' + format(i, '#04x') + '\n')

        row = TABLE_ROW.format(func(i), func(i + 1), func(i + 2), func(i + 3))
        output.append(row)

    output.append('];')

    output.append(TABLE_DECL.format('CB_OPCODE'))

    for i in xrange(0, 256, 4):
        if i % 16 == 0:
            output.append('    // ' + format(i, '#04x') + '\n')

        row = TABLE_ROW.format(func_cb(i), func_cb(
            i + 1), func_cb(i + 2), func_cb(i + 3))
        output.append(row)

    output.append('];')

    return output


def build_disasm_metadata():
    '''
        Generates the source file for mapping opcodes their names.

        The generated file contains two static arrays of &str
        that map the opcode (a byte value) to the name of that instruction.

        Returns a list of strings that makes up the generated source file
    '''

    output = []
    output.append(DISCLAIMER)
    output.append(DISASM_T_DECL.format('DISASM'))

    for i in xrange(0, 256, 4):
        if i % 16 == 0:
            output.append('    // ' + format(i, '#04x') + '\n')

        row = DISASM_T_ROW.format(disasm(i), disasm(
            i + 1), disasm(i + 2), disasm(i + 3))
        output.append(row)

    output.append('];')
    output.append(DISASM_T_DECL.format('CB_DISASM'))

    for i in xrange(0, 256, 4):
        if i % 16 == 0:
            output.append('    // ' + format(i, '#04x') + '\n')

        row = DISASM_T_ROW.format(disasm_cb(i), disasm_cb(
            i + 1), disasm_cb(i + 2), disasm_cb(i + 3))
        output.append(row)

    output.append('];')
    return output


def func(opcode):
    return CODES[opcode]['fn'] if opcode in CODES else UNIMPLEMENTED


def func_cb(opcode):
    return CB_CODES[opcode]['fn'] if opcode in CB_CODES else UNIMPLEMENTED_CB


def disasm(opcode):
    return CODES[opcode]['name'] if opcode in CODES else 'Missing'


def disasm_cb(opcode):
    return CB_CODES[opcode]['name'] if opcode in CB_CODES else 'Missing'

if __name__ == "__main__":
    for src_file in glob.glob('src/jeebie/instr/*.rs'):
        process_opcode_file(src_file)

    with open('src/jeebie/instr/opcodes.rs', 'w') as out_file:
        out_file.writelines(build_output())

    with open('src/jeebie/disasm/metadata.rs', 'w') as out_file:
        out_file.writelines(build_disasm_metadata())

    print "Done generating mapping files."
