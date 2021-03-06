use jeebie::core::cpu::CPU;
use jeebie::core::registers::Register8::*;
use jeebie::core::registers::Register16::*;

// 'LD B,n' 06 8
pub fn LD_B_n(cpu: &mut CPU) -> i32 {
    cpu.load_rr(B, N); 8
}

// 'LD C,n' 0E 8
pub fn LD_C_n(cpu: &mut CPU) -> i32 {
    cpu.load_rr(C, N);
    8
}

// 'LD D,n' 16 8
pub fn LD_D_n(cpu: &mut CPU) -> i32 {
    cpu.load_rr(D, N);
    8
}

// 'LD E,n' 1E 8
pub fn LD_E_n(cpu: &mut CPU) -> i32 {
    cpu.load_rr(E, N);
    8
}

// 'LD H,n' 26 8
pub fn LD_H_n(cpu: &mut CPU) -> i32 {
    cpu.load_rr(H, N);
    8
}

// 'LD L,n' 2E 8
pub fn LD_L_n(cpu: &mut CPU) -> i32 {
    cpu.load_rr(L, N);
    8
}

// 'LD A,A' 7F 4
pub fn LD_a_a(cpu: &mut CPU) -> i32 {
    cpu.load_rr(A, A);
    4
}

// 'LD A,B' 78 4
pub fn LD_a_b(cpu: &mut CPU) -> i32 {
    cpu.load_rr(A, B);
    4
}

// 'LD A,C' 79 4
pub fn LD_a_c(cpu: &mut CPU) -> i32 {
    cpu.load_rr(A, C);
    4
}

// 'LD A,D' 7A 4
pub fn LD_a_d(cpu: &mut CPU) -> i32 {
    cpu.load_rr(A, D);
    4
}

// 'LD A,E' 7B 4
pub fn LD_a_e(cpu: &mut CPU) -> i32 {
    cpu.load_rr(A, E);
    4
}

// 'LD A,H' 7C 4
pub fn LD_a_h(cpu: &mut CPU) -> i32 {
    cpu.load_rr(A, H);
    4
}

// 'LD A,L' 7D 4
pub fn LD_a_l(cpu: &mut CPU) -> i32 {
    cpu.load_rr(A, L);
    4
}

// 'LD B,B' 40 4
pub fn LD_b_b(cpu: &mut CPU) -> i32 {
    cpu.load_rr(B, B);
    4
}
// 'LD B,C' 41 4
pub fn LD_b_c(cpu: &mut CPU) -> i32 {
    cpu.load_rr(B, C);
    4
}

// 'LD B,D' 42 4
pub fn LD_b_d(cpu: &mut CPU) -> i32 {
    cpu.load_rr(B, D);
    4
}

// 'LD B,E' 43 4
pub fn LD_b_e(cpu: &mut CPU) -> i32 {
    cpu.load_rr(B, E);
    4
}

// 'LD B,H' 44 4
pub fn LD_b_h(cpu: &mut CPU) -> i32 {
    cpu.load_rr(B, H);
    4
}

// 'LD B,L' 45 4
pub fn LD_b_l(cpu: &mut CPU) -> i32 {
    cpu.load_rr(B, L);
    4
}

// 'LD B,A' 47 4
pub fn LD_b_a(cpu: &mut CPU) -> i32 {
    cpu.load_rr(B, A);
    4
}

// 'LD B,(HL)' 46 8
pub fn LD_b_HLm(cpu: &mut CPU) -> i32 {
    cpu.load_rr(B, RegisterAddress(HL));
    8
}

// 'LD C,B' 48 4
pub fn LD_c_b(cpu: &mut CPU) -> i32 {
    cpu.load_rr(C, B);
    4
}

// 'LD C,C' 49 4
pub fn LD_c_c(cpu: &mut CPU) -> i32 {
    cpu.load_rr(C, C);
    4
}

// 'LD C,D' 4A 4
pub fn LD_c_d(cpu: &mut CPU) -> i32 {
    cpu.load_rr(C, D);
    4
}

// 'LD C,E' 4B 4
pub fn LD_c_e(cpu: &mut CPU) -> i32 {
    cpu.load_rr(C, E);
    4
}

// 'LD C,H' 4C 4
pub fn LD_c_h(cpu: &mut CPU) -> i32 {
    cpu.load_rr(C, H);
    4
}

// 'LD C,L' 4D 4
pub fn LD_c_l(cpu: &mut CPU) -> i32 {
    cpu.load_rr(C, L);
    4
}

// 'LD C,A' 4F 4
pub fn LD_c_a(cpu: &mut CPU) -> i32 {
    cpu.load_rr(C, A);
    4
}

// 'LD C,(HL)' 4E 8
pub fn LD_c_HLm(cpu: &mut CPU) -> i32 {
    cpu.load_rr(C, RegisterAddress(HL));
    8
}

// 'LD D,B' 50 4
pub fn LD_d_b(cpu: &mut CPU) -> i32 {
    cpu.load_rr(D, B);
    4
}

// 'LD D,C' 51 4
pub fn LD_d_c(cpu: &mut CPU) -> i32 {
    cpu.load_rr(D, C);
    4
}

// 'LD D,D' 52 4
pub fn LD_d_e(cpu: &mut CPU) -> i32 {
    cpu.load_rr(D, D);
    4
}

// 'LD D,E' 53 4
pub fn LD_d_d(cpu: &mut CPU) -> i32 {
    cpu.load_rr(D, E);
    4
}

// 'LD D,H' 54 4
pub fn LD_d_h(cpu: &mut CPU) -> i32 {
    cpu.load_rr(D, H);
    4
}

// 'LD D,L' 55 4
pub fn LD_d_l(cpu: &mut CPU) -> i32 {
    cpu.load_rr(D, L);
    4
}

// 'LD D,A' 57 4
pub fn LD_d_a(cpu: &mut CPU) -> i32 {
    cpu.load_rr(D, A);
    4
}

// 'LD D,(HL)' 56 8
pub fn LD_d_HLm(cpu: &mut CPU) -> i32 {
    cpu.load_rr(D, RegisterAddress(HL));
    8
}

// 'LD E,B' 58 4
pub fn LD_e_b(cpu: &mut CPU) -> i32 {
    cpu.load_rr(E, B);
    4
}

// 'LD E,C' 59 4
pub fn LD_e_c(cpu: &mut CPU) -> i32 {
    cpu.load_rr(E, C);
    4
}

// 'LD E,D' 5A 4
pub fn LD_e_d(cpu: &mut CPU) -> i32 {
    cpu.load_rr(E, D);
    4
}

// 'LD E,E' 5B 4
pub fn LD_e_e(cpu: &mut CPU) -> i32 {
    cpu.load_rr(E, E);
    4
}

// 'LD E,H' 5C 4
pub fn LD_e_h(cpu: &mut CPU) -> i32 {
    cpu.load_rr(E, H);
    4
}

// 'LD E,L' 5D 4
pub fn LD_e_l(cpu: &mut CPU) -> i32 {
    cpu.load_rr(E, L);
    4
}

// 'LD E,(HL)' 5E 8
pub fn LD_e_HLm(cpu: &mut CPU) -> i32 {
    cpu.load_rr(E, RegisterAddress(HL));
    8
}

// 'LD E,A' 5F 4
pub fn LD_e_a(cpu: &mut CPU) -> i32 {
    cpu.load_rr(E, A);
    4
}

// 'LD H,B' 60 4
pub fn LD_h_b(cpu: &mut CPU) -> i32 {
    cpu.load_rr(H, B);
    4
}

// 'LD H,C' 61 4
pub fn LD_h_c(cpu: &mut CPU) -> i32 {
    cpu.load_rr(H, C);
    4
}

// 'LD H,D' 62 4
pub fn LD_h_d(cpu: &mut CPU) -> i32 {
    cpu.load_rr(H, D);
    4
}

// 'LD H,E' 63 4
pub fn LD_h_e(cpu: &mut CPU) -> i32 {
    cpu.load_rr(H, E);
    4
}

// 'LD H,H' 64 4
pub fn LD_h_h(cpu: &mut CPU) -> i32 {
    cpu.load_rr(H, H);
    4
}

// 'LD H,L' 65 4
pub fn LD_h_l(cpu: &mut CPU) -> i32 {
    cpu.load_rr(H, L);
    4
}

// 'LD H,(HL)' 66 8
pub fn LD_h_HLm(cpu: &mut CPU) -> i32 {
    cpu.load_rr(H, RegisterAddress(HL));
    8
}

// 'LD H,A' 67 4
pub fn LD_h_a(cpu: &mut CPU) -> i32 {
    cpu.load_rr(H, A);
    4
}

// 'LD L,B' 68 4
pub fn LD_l_b(cpu: &mut CPU) -> i32 {
    cpu.load_rr(L, B);
    4
}

// 'LD L,C' 69 4
pub fn LD_l_c(cpu: &mut CPU) -> i32 {
    cpu.load_rr(L, C);
    4
}

// 'LD L,D' 6A 4
pub fn LD_l_d(cpu: &mut CPU) -> i32 {
    cpu.load_rr(L, D);
    4
}

// 'LD L,E' 6B 4
pub fn LD_l_e(cpu: &mut CPU) -> i32 {
    cpu.load_rr(L, E);
    4
}

// 'LD L,H' 6C 4
pub fn LD_l_h(cpu: &mut CPU) -> i32 {
    cpu.load_rr(L, H);
    4
}

// 'LD L,L' 6D 4
pub fn LD_l_l(cpu: &mut CPU) -> i32 {
    cpu.load_rr(L, L);
    4
}

// 'LD L,A' 6F 4
pub fn LD_l_a(cpu: &mut CPU) -> i32 {
    cpu.load_rr(L, A);
    4
}

// 'LD L,(HL)' 6E 8
pub fn LD_l_HLm(cpu: &mut CPU) -> i32 {
    cpu.load_rr(L, RegisterAddress(HL));
    8
}

// 'LD (HL),B' 70 8
pub fn LD_HLm_b(cpu: &mut CPU) -> i32 {
    cpu.load_rr(RegisterAddress(HL), B);
    8
}

// 'LD (HL),C' 71 8
pub fn LD_HLm_c(cpu: &mut CPU) -> i32 {
    cpu.load_rr(RegisterAddress(HL), C);
    8
}

// 'LD (HL),D' 72 8
pub fn LD_HLm_d(cpu: &mut CPU) -> i32 {
    cpu.load_rr(RegisterAddress(HL), D);
    8
}

// 'LD (HL),E' 73 8
pub fn LD_HLm_e(cpu: &mut CPU) -> i32 {
    cpu.load_rr(RegisterAddress(HL), E);
    8
}

// 'LD (HL),H' 74 8
pub fn LD_HLm_h(cpu: &mut CPU) -> i32 {
    cpu.load_rr(RegisterAddress(HL), H);
    8
}

// 'LD (HL),L' 75 8
pub fn LD_HLm_l(cpu: &mut CPU) -> i32 {
    cpu.load_rr(RegisterAddress(HL), L);
    8
}

// 'LD (HL),n' 36 12
pub fn LD_HLm_n(cpu: &mut CPU) -> i32 {
    cpu.load_rr(RegisterAddress(HL), N);
    2
}

// 'LD (HL),A' 77 8
pub fn LD_HLm_a(cpu: &mut CPU) -> i32 {
    cpu.load_rr(RegisterAddress(HL), A);
    8
}

// 'LD A,(BC)' 0A 8
pub fn LD_a_BCm(cpu: &mut CPU) -> i32 {
    cpu.load_rr(A, RegisterAddress(BC));
    8
}

// 'LD A,(DE)' 1A 8
pub fn LD_a_DEm(cpu: &mut CPU) -> i32 {
    cpu.load_rr(A, RegisterAddress(DE));
    8
}

// 'LD A,(HL)' 7E 8
pub fn LD_a_HLm(cpu: &mut CPU) -> i32 {
    cpu.load_rr(A, RegisterAddress(HL));
    8
}

// 'LD A,(nn)' FA 16
pub fn LD_a_nnm(cpu: &mut CPU) -> i32 {
    cpu.load_rr(A, RegisterAddress(NN));
    6
}

// 'LD A,n' 3E 8
pub fn LD_a_n(cpu: &mut CPU) -> i32 {
    cpu.load_rr(A, N);
    8
}

// 'LD (BC),A' 02 8
pub fn LD_BCm_A(cpu: &mut CPU) -> i32 {
    cpu.load_rr(RegisterAddress(BC), A);
    8
}

// 'LD (DE),A' 12 8
pub fn LD_DEm_A(cpu: &mut CPU) -> i32 {
    cpu.load_rr(RegisterAddress(DE), A);
    8
}

// 'LD (nn),A' EA 16
pub fn LD_nnm_A(cpu: &mut CPU) -> i32 {
    cpu.load_rr(RegisterAddress(NN), A);
    6
}

// 'LD A,($FF00 + C)' F2 8
pub fn LD_a_c_mem(cpu: &mut CPU) -> i32 {
    let addr = 0xFF00 | (cpu.get8(C) as u16);
    cpu.load_rr(A, Address(addr));
    8
}

// 'LD ($FF00+C),A' E2 8
pub fn LD_c_mem_a(cpu: &mut CPU) -> i32 {
    let addr = 0xFF00 | (cpu.get8(C) as u16);
    cpu.load_rr(Address(addr), A);
    8
}

// 'LDD A,(HL)' 3A 8
pub fn LDD_a_HLm(cpu: &mut CPU) -> i32 {
    cpu.load_rr(A, RegisterAddress(HL));
    cpu.compute_dec16(HL);
    8
}

// 'LDD (HL),A' 32 8
pub fn LDD_HLm_a(cpu: &mut CPU) -> i32 {
    cpu.load_rr(RegisterAddress(HL), A);
    cpu.compute_dec16(HL);
    8
}

// 'LDI A,(HL)' 2A 8
pub fn LDI_a_HLm(cpu: &mut CPU) -> i32 {
    cpu.load_rr(A, RegisterAddress(HL));
    cpu.compute_inc16(HL);
    8
}

// 'LDI (HL),A' 22 8
pub fn LDI_HLm_a(cpu: &mut CPU) -> i32 {
    cpu.load_rr(RegisterAddress(HL), A);
    cpu.compute_inc16(HL);
    8
}

// 'LDH ($FF00+n),A' E0 12
pub fn LDH_nm_a(cpu: &mut CPU) -> i32 {
    let addr = 0xFF00 | (cpu.get8(N) as u16);
    cpu.load_rr(Address(addr), A);
    12
}

// 'LDH A,($FF00+n)' F0 12
pub fn LDH_a_nm(cpu: &mut CPU) -> i32 {
    let addr = 0xFF00 | (cpu.get8(N) as u16);
    cpu.load_rr(A, Address(addr));
    12
}

// *** 16-bit loads ***

// 'LD BC,nn' 01 12
pub fn LD_bc_nn(cpu: &mut CPU) -> i32 {
    cpu.load_rr16(BC, NN);
    12
}

// 'LD DE,nn' 11 12
pub fn LD_de_nn(cpu: &mut CPU) -> i32 {
    cpu.load_rr16(DE, NN);
    12
}

// 'LD HL,nn' 21 12
pub fn LD_hl_nn(cpu: &mut CPU) -> i32 {
    cpu.load_rr16(HL, NN);
    12
}

// 'LD SP,nn' 31 12
pub fn LD_sp_nn(cpu: &mut CPU) -> i32 {
    cpu.load_rr16(SP, NN);
    12
}

// 'LD SP,HL' F9 8
pub fn LD_sp_hl(cpu: &mut CPU) -> i32 {
    cpu.load_rr16(SP, HL);
    8
}

// 'LD HL,SP+n' F8 12
pub fn LDHL_sp_n(cpu: &mut CPU) -> i32 {
    let spn = cpu.get16(SP).wrapping_add(cpu.get8(N) as u16);
    cpu.compute_add16(HL, Value16(spn));
    12
}

// 'LD (nn),SP' 08 20
pub fn LD_nnm_sp(cpu: &mut CPU) -> i32 {
    let addr_low = cpu.get_immediate16();
    let addr_high = addr_low.wrapping_add(1);

    let low = (cpu.get16(SP) & 0x00FF) as u8;
    let high = (cpu.get16(SP) >> 8) as u8;

    cpu.load_rr(Address(addr_low), Value8(low));
    cpu.load_rr(Address(addr_high), Value8(high));
    20
}
