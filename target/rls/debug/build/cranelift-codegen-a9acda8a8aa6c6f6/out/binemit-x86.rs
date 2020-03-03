/// Emit binary machine code for `inst` for the x86 ISA.
#[allow(unused_variables, unreachable_code)]
pub fn emit_inst<CS: CodeSink + ?Sized>(
    func: &Function,
    inst: Inst,
    divert: &mut RegDiversions,
    sink: &mut CS,
    isa: &dyn TargetIsa,
) {
    let encoding = func.encodings[inst];
    let bits = encoding.bits();
    let inst_data = &func.dfg[inst];
    match encoding.recipe() {
        // Recipe get_pinned_reg
        0 => {
            if let InstructionData::NullAry {
                opcode,
                ..
            } = *inst_data {
                return;
            }
        }
        // Recipe RexOp1set_pinned_reg
        1 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let r15 = RU::r15.into();
                put_rexop1(bits, rex2(r15, in_reg0), sink);
                modrm_rr(r15, in_reg0, sink);
                return;
            }
        }
        // Recipe Op1rr
        2 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_op1(bits, rex2(in_reg0, in_reg1), sink);
                modrm_rr(in_reg0, in_reg1, sink);
                return;
            }
        }
        // Recipe RexOp1rr
        3 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_rexop1(bits, rex2(in_reg0, in_reg1), sink);
                modrm_rr(in_reg0, in_reg1, sink);
                return;
            }
        }
        // Recipe Op1rout
        4 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_op1(bits, rex2(in_reg0, in_reg1), sink);
                modrm_rr(in_reg0, in_reg1, sink);
                return;
            }
        }
        // Recipe RexOp1rout
        5 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_rexop1(bits, rex2(in_reg0, in_reg1), sink);
                modrm_rr(in_reg0, in_reg1, sink);
                return;
            }
        }
        // Recipe Op1rin
        6 => {
            if let InstructionData::Ternary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_op1(bits, rex2(in_reg0, in_reg1), sink);
                modrm_rr(in_reg0, in_reg1, sink);
                return;
            }
        }
        // Recipe RexOp1rin
        7 => {
            if let InstructionData::Ternary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_rexop1(bits, rex2(in_reg0, in_reg1), sink);
                modrm_rr(in_reg0, in_reg1, sink);
                return;
            }
        }
        // Recipe Op1rio
        8 => {
            if let InstructionData::Ternary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_op1(bits, rex2(in_reg0, in_reg1), sink);
                modrm_rr(in_reg0, in_reg1, sink);
                return;
            }
        }
        // Recipe RexOp1rio
        9 => {
            if let InstructionData::Ternary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_rexop1(bits, rex2(in_reg0, in_reg1), sink);
                modrm_rr(in_reg0, in_reg1, sink);
                return;
            }
        }
        // Recipe Op1ur
        10 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                put_op1(bits, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                return;
            }
        }
        // Recipe RexOp1ur
        11 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                put_rexop1(bits, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                return;
            }
        }
        // Recipe Op2rrx
        12 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_op2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                return;
            }
        }
        // Recipe RexOp2rrx
        13 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_rexop2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                return;
            }
        }
        // Recipe Op1div
        14 => {
            if let InstructionData::Ternary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg2 = divert.reg(args[2], &func.locations);
                sink.trap(TrapCode::IntegerDivisionByZero, func.srclocs[inst]);
                put_op1(bits, rex1(in_reg2), sink);
                modrm_r_bits(in_reg2, bits, sink);
                return;
            }
        }
        // Recipe RexOp1div
        15 => {
            if let InstructionData::Ternary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg2 = divert.reg(args[2], &func.locations);
                sink.trap(TrapCode::IntegerDivisionByZero, func.srclocs[inst]);
                put_rexop1(bits, rex1(in_reg2), sink);
                modrm_r_bits(in_reg2, bits, sink);
                return;
            }
        }
        // Recipe Op1mulx
        16 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_op1(bits, rex1(in_reg1), sink);
                modrm_r_bits(in_reg1, bits, sink);
                return;
            }
        }
        // Recipe RexOp1mulx
        17 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_rexop1(bits, rex1(in_reg1), sink);
                modrm_r_bits(in_reg1, bits, sink);
                return;
            }
        }
        // Recipe Op1umr
        18 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_op1(bits, rex2(out_reg0, in_reg0), sink);
                modrm_rr(out_reg0, in_reg0, sink);
                return;
            }
        }
        // Recipe RexOp1umr
        19 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop1(bits, rex2(out_reg0, in_reg0), sink);
                modrm_rr(out_reg0, in_reg0, sink);
                return;
            }
        }
        // Recipe Op1rmov
        20 => {
            if let InstructionData::RegMove {
                opcode,
                src,
                dst,
                arg,
                ..
            } = *inst_data {
                divert.apply(inst_data);
                put_op1(bits, rex2(dst, src), sink);
                modrm_rr(dst, src, sink);
                return;
            }
        }
        // Recipe RexOp1rmov
        21 => {
            if let InstructionData::RegMove {
                opcode,
                src,
                dst,
                arg,
                ..
            } = *inst_data {
                divert.apply(inst_data);
                put_rexop1(bits, rex2(dst, src), sink);
                modrm_rr(dst, src, sink);
                return;
            }
        }
        // Recipe Op1r_ib
        22 => {
            if let InstructionData::BinaryImm {
                opcode,
                imm,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                put_op1(bits, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                let imm: i64 = imm.into();
                sink.put1(imm as u8);
                return;
            }
        }
        // Recipe RexOp1r_ib
        23 => {
            if let InstructionData::BinaryImm {
                opcode,
                imm,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                put_rexop1(bits, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                let imm: i64 = imm.into();
                sink.put1(imm as u8);
                return;
            }
        }
        // Recipe Op1r_id
        24 => {
            if let InstructionData::BinaryImm {
                opcode,
                imm,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                put_op1(bits, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                let imm: i64 = imm.into();
                sink.put4(imm as u32);
                return;
            }
        }
        // Recipe RexOp1r_id
        25 => {
            if let InstructionData::BinaryImm {
                opcode,
                imm,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                put_rexop1(bits, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                let imm: i64 = imm.into();
                sink.put4(imm as u32);
                return;
            }
        }
        // Recipe Op1pu_id
        26 => {
            if let InstructionData::UnaryImm {
                opcode,
                imm,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                // The destination register is encoded in the low bits of the opcode.
                // No ModR/M.
                put_op1(bits | (out_reg0 & 7), rex1(out_reg0), sink);
                let imm: i64 = imm.into();
                sink.put4(imm as u32);
                return;
            }
        }
        // Recipe RexOp1pu_id
        27 => {
            if let InstructionData::UnaryImm {
                opcode,
                imm,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                // The destination register is encoded in the low bits of the opcode.
                // No ModR/M.
                put_rexop1(bits | (out_reg0 & 7), rex1(out_reg0), sink);
                let imm: i64 = imm.into();
                sink.put4(imm as u32);
                return;
            }
        }
        // Recipe RexOp1u_id
        28 => {
            if let InstructionData::UnaryImm {
                opcode,
                imm,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop1(bits, rex1(out_reg0), sink);
                modrm_r_bits(out_reg0, bits, sink);
                let imm: i64 = imm.into();
                sink.put4(imm as u32);
                return;
            }
        }
        // Recipe RexOp1pu_iq
        29 => {
            if let InstructionData::UnaryImm {
                opcode,
                imm,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop1(bits | (out_reg0 & 7), rex1(out_reg0), sink);
                let imm: i64 = imm.into();
                sink.put8(imm as u64);
                return;
            }
        }
        // Recipe Op1pu_id_bool
        30 => {
            if let InstructionData::UnaryBool {
                opcode,
                imm,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                // The destination register is encoded in the low bits of the opcode.
                // No ModR/M.
                put_op1(bits | (out_reg0 & 7), rex1(out_reg0), sink);
                let imm: u32 = if imm { 1 } else { 0 };
                sink.put4(imm);
                return;
            }
        }
        // Recipe RexOp1pu_id_bool
        31 => {
            if let InstructionData::UnaryBool {
                opcode,
                imm,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                // The destination register is encoded in the low bits of the opcode.
                // No ModR/M.
                put_rexop1(bits | (out_reg0 & 7), rex1(out_reg0), sink);
                let imm: u32 = if imm { 1 } else { 0 };
                sink.put4(imm);
                return;
            }
        }
        // Recipe Op1u_id_z
        32 => {
            if let InstructionData::UnaryImm {
                opcode,
                imm,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_op1(bits, rex2(out_reg0, out_reg0), sink);
                modrm_rr(out_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe RexOp1u_id_z
        33 => {
            if let InstructionData::UnaryImm {
                opcode,
                imm,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop1(bits, rex2(out_reg0, out_reg0), sink);
                modrm_rr(out_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe Op1rc
        34 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                put_op1(bits, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                return;
            }
        }
        // Recipe RexOp1rc
        35 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                put_rexop1(bits, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                return;
            }
        }
        // Recipe Mp2urm
        36 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_mp2(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe RexMp2urm
        37 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexmp2(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe Op1ldWithIndex
        38 => {
            if let InstructionData::LoadComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op1(bits, rex3(in_reg0, out_reg0, in_reg1), sink);
                // The else branch always inserts an SIB byte.
                if needs_offset(in_reg0) {
                    modrm_sib_disp8(out_reg0, sink);
                    sib(0, in_reg1, in_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_sib(out_reg0, sink);
                    sib(0, in_reg1, in_reg0, sink);
                }
                return;
            }
        }
        // Recipe RexOp1ldWithIndex
        39 => {
            if let InstructionData::LoadComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop1(bits, rex3(in_reg0, out_reg0, in_reg1), sink);
                // The else branch always inserts an SIB byte.
                if needs_offset(in_reg0) {
                    modrm_sib_disp8(out_reg0, sink);
                    sib(0, in_reg1, in_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_sib(out_reg0, sink);
                    sib(0, in_reg1, in_reg0, sink);
                }
                return;
            }
        }
        // Recipe Op2ldWithIndex
        40 => {
            if let InstructionData::LoadComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op2(bits, rex3(in_reg0, out_reg0, in_reg1), sink);
                // The else branch always inserts an SIB byte.
                if needs_offset(in_reg0) {
                    modrm_sib_disp8(out_reg0, sink);
                    sib(0, in_reg1, in_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_sib(out_reg0, sink);
                    sib(0, in_reg1, in_reg0, sink);
                }
                return;
            }
        }
        // Recipe RexOp2ldWithIndex
        41 => {
            if let InstructionData::LoadComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop2(bits, rex3(in_reg0, out_reg0, in_reg1), sink);
                // The else branch always inserts an SIB byte.
                if needs_offset(in_reg0) {
                    modrm_sib_disp8(out_reg0, sink);
                    sib(0, in_reg1, in_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_sib(out_reg0, sink);
                    sib(0, in_reg1, in_reg0, sink);
                }
                return;
            }
        }
        // Recipe Op1ldWithIndexDisp8
        42 => {
            if let InstructionData::LoadComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op1(bits, rex3(in_reg0, out_reg0, in_reg1), sink);
                modrm_sib_disp8(out_reg0, sink);
                sib(0, in_reg1, in_reg0, sink);
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe RexOp1ldWithIndexDisp8
        43 => {
            if let InstructionData::LoadComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop1(bits, rex3(in_reg0, out_reg0, in_reg1), sink);
                modrm_sib_disp8(out_reg0, sink);
                sib(0, in_reg1, in_reg0, sink);
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe Op2ldWithIndexDisp8
        44 => {
            if let InstructionData::LoadComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op2(bits, rex3(in_reg0, out_reg0, in_reg1), sink);
                modrm_sib_disp8(out_reg0, sink);
                sib(0, in_reg1, in_reg0, sink);
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe RexOp2ldWithIndexDisp8
        45 => {
            if let InstructionData::LoadComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop2(bits, rex3(in_reg0, out_reg0, in_reg1), sink);
                modrm_sib_disp8(out_reg0, sink);
                sib(0, in_reg1, in_reg0, sink);
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe Op1ldWithIndexDisp32
        46 => {
            if let InstructionData::LoadComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op1(bits, rex3(in_reg0, out_reg0, in_reg1), sink);
                modrm_sib_disp32(out_reg0, sink);
                sib(0, in_reg1, in_reg0, sink);
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe RexOp1ldWithIndexDisp32
        47 => {
            if let InstructionData::LoadComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop1(bits, rex3(in_reg0, out_reg0, in_reg1), sink);
                modrm_sib_disp32(out_reg0, sink);
                sib(0, in_reg1, in_reg0, sink);
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe Op2ldWithIndexDisp32
        48 => {
            if let InstructionData::LoadComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op2(bits, rex3(in_reg0, out_reg0, in_reg1), sink);
                modrm_sib_disp32(out_reg0, sink);
                sib(0, in_reg1, in_reg0, sink);
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe RexOp2ldWithIndexDisp32
        49 => {
            if let InstructionData::LoadComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop2(bits, rex3(in_reg0, out_reg0, in_reg1), sink);
                modrm_sib_disp32(out_reg0, sink);
                sib(0, in_reg1, in_reg0, sink);
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe Op1stWithIndex
        50 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op1(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                // The else branch always inserts an SIB byte.
                if needs_offset(in_reg1) {
                    modrm_sib_disp8(in_reg0, sink);
                    sib(0, in_reg2, in_reg1, sink);
                    sink.put1(0);
                } else {
                    modrm_sib(in_reg0, sink);
                    sib(0, in_reg2, in_reg1, sink);
                }
                return;
            }
        }
        // Recipe RexOp1stWithIndex
        51 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop1(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                // The else branch always inserts an SIB byte.
                if needs_offset(in_reg1) {
                    modrm_sib_disp8(in_reg0, sink);
                    sib(0, in_reg2, in_reg1, sink);
                    sink.put1(0);
                } else {
                    modrm_sib(in_reg0, sink);
                    sib(0, in_reg2, in_reg1, sink);
                }
                return;
            }
        }
        // Recipe Mp1stWithIndex
        52 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_mp1(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                // The else branch always inserts an SIB byte.
                if needs_offset(in_reg1) {
                    modrm_sib_disp8(in_reg0, sink);
                    sib(0, in_reg2, in_reg1, sink);
                    sink.put1(0);
                } else {
                    modrm_sib(in_reg0, sink);
                    sib(0, in_reg2, in_reg1, sink);
                }
                return;
            }
        }
        // Recipe RexMp1stWithIndex
        53 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexmp1(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                // The else branch always inserts an SIB byte.
                if needs_offset(in_reg1) {
                    modrm_sib_disp8(in_reg0, sink);
                    sib(0, in_reg2, in_reg1, sink);
                    sink.put1(0);
                } else {
                    modrm_sib(in_reg0, sink);
                    sib(0, in_reg2, in_reg1, sink);
                }
                return;
            }
        }
        // Recipe Op1stWithIndexDisp8
        54 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op1(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                modrm_sib_disp8(in_reg0, sink);
                sib(0, in_reg2, in_reg1, sink);
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe RexOp1stWithIndexDisp8
        55 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop1(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                modrm_sib_disp8(in_reg0, sink);
                sib(0, in_reg2, in_reg1, sink);
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe Mp1stWithIndexDisp8
        56 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_mp1(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                modrm_sib_disp8(in_reg0, sink);
                sib(0, in_reg2, in_reg1, sink);
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe RexMp1stWithIndexDisp8
        57 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexmp1(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                modrm_sib_disp8(in_reg0, sink);
                sib(0, in_reg2, in_reg1, sink);
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe Op1stWithIndexDisp32
        58 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op1(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                modrm_sib_disp32(in_reg0, sink);
                sib(0, in_reg2, in_reg1, sink);
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe RexOp1stWithIndexDisp32
        59 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop1(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                modrm_sib_disp32(in_reg0, sink);
                sib(0, in_reg2, in_reg1, sink);
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe Mp1stWithIndexDisp32
        60 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_mp1(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                modrm_sib_disp32(in_reg0, sink);
                sib(0, in_reg2, in_reg1, sink);
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe RexMp1stWithIndexDisp32
        61 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexmp1(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                modrm_sib_disp32(in_reg0, sink);
                sib(0, in_reg2, in_reg1, sink);
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe Op1stWithIndex_abcd
        62 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op1(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                // The else branch always inserts an SIB byte.
                if needs_offset(in_reg1) {
                    modrm_sib_disp8(in_reg0, sink);
                    sib(0, in_reg2, in_reg1, sink);
                    sink.put1(0);
                } else {
                    modrm_sib(in_reg0, sink);
                    sib(0, in_reg2, in_reg1, sink);
                }
                return;
            }
        }
        // Recipe RexOp1stWithIndex_abcd
        63 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop1(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                // The else branch always inserts an SIB byte.
                if needs_offset(in_reg1) {
                    modrm_sib_disp8(in_reg0, sink);
                    sib(0, in_reg2, in_reg1, sink);
                    sink.put1(0);
                } else {
                    modrm_sib(in_reg0, sink);
                    sib(0, in_reg2, in_reg1, sink);
                }
                return;
            }
        }
        // Recipe Op1stWithIndexDisp8_abcd
        64 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op1(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                modrm_sib_disp8(in_reg0, sink);
                sib(0, in_reg2, in_reg1, sink);
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe RexOp1stWithIndexDisp8_abcd
        65 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop1(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                modrm_sib_disp8(in_reg0, sink);
                sib(0, in_reg2, in_reg1, sink);
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe Op1stWithIndexDisp32_abcd
        66 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op1(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                modrm_sib_disp32(in_reg0, sink);
                sib(0, in_reg2, in_reg1, sink);
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe RexOp1stWithIndexDisp32_abcd
        67 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop1(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                modrm_sib_disp32(in_reg0, sink);
                sib(0, in_reg2, in_reg1, sink);
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe Op1st
        68 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op1(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else if needs_offset(in_reg1) {
                    modrm_disp8(in_reg1, in_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_rm(in_reg1, in_reg0, sink);
                }
                return;
            }
        }
        // Recipe RexOp1st
        69 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop1(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else if needs_offset(in_reg1) {
                    modrm_disp8(in_reg1, in_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_rm(in_reg1, in_reg0, sink);
                }
                return;
            }
        }
        // Recipe Mp1st
        70 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_mp1(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else if needs_offset(in_reg1) {
                    modrm_disp8(in_reg1, in_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_rm(in_reg1, in_reg0, sink);
                }
                return;
            }
        }
        // Recipe RexMp1st
        71 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexmp1(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else if needs_offset(in_reg1) {
                    modrm_disp8(in_reg1, in_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_rm(in_reg1, in_reg0, sink);
                }
                return;
            }
        }
        // Recipe Op1stDisp8
        72 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op1(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib_disp8(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else {
                    modrm_disp8(in_reg1, in_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe RexOp1stDisp8
        73 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop1(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib_disp8(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else {
                    modrm_disp8(in_reg1, in_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe Mp1stDisp8
        74 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_mp1(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib_disp8(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else {
                    modrm_disp8(in_reg1, in_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe RexMp1stDisp8
        75 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexmp1(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib_disp8(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else {
                    modrm_disp8(in_reg1, in_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe Op1stDisp32
        76 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op1(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib_disp32(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else {
                    modrm_disp32(in_reg1, in_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe RexOp1stDisp32
        77 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop1(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib_disp32(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else {
                    modrm_disp32(in_reg1, in_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe Mp1stDisp32
        78 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_mp1(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib_disp32(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else {
                    modrm_disp32(in_reg1, in_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe RexMp1stDisp32
        79 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexmp1(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib_disp32(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else {
                    modrm_disp32(in_reg1, in_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe Op1st_abcd
        80 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op1(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else if needs_offset(in_reg1) {
                    modrm_disp8(in_reg1, in_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_rm(in_reg1, in_reg0, sink);
                }
                return;
            }
        }
        // Recipe Op1stDisp8_abcd
        81 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op1(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib_disp8(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else {
                    modrm_disp8(in_reg1, in_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe Op1stDisp32_abcd
        82 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op1(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib_disp32(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else {
                    modrm_disp32(in_reg1, in_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe Op1spillSib32
        83 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_stk0 = StackRef::masked(
                    divert.stack(results[0], &func.locations),
                    StackBaseMask(1),
                    &func.stack_slots,
                ).unwrap();
                sink.trap(TrapCode::StackOverflow, func.srclocs[inst]);
                let base = stk_base(out_stk0.base);
                put_op1(bits, rex2(base, in_reg0), sink);
                modrm_sib_disp32(in_reg0, sink);
                sib_noindex(base, sink);
                sink.put4(out_stk0.offset as u32);
                return;
            }
        }
        // Recipe RexOp1spillSib32
        84 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_stk0 = StackRef::masked(
                    divert.stack(results[0], &func.locations),
                    StackBaseMask(1),
                    &func.stack_slots,
                ).unwrap();
                sink.trap(TrapCode::StackOverflow, func.srclocs[inst]);
                let base = stk_base(out_stk0.base);
                put_rexop1(bits, rex2(base, in_reg0), sink);
                modrm_sib_disp32(in_reg0, sink);
                sib_noindex(base, sink);
                sink.put4(out_stk0.offset as u32);
                return;
            }
        }
        // Recipe Op1regspill32
        85 => {
            if let InstructionData::RegSpill {
                opcode,
                src,
                dst,
                arg,
                ..
            } = *inst_data {
                divert.apply(inst_data);
                sink.trap(TrapCode::StackOverflow, func.srclocs[inst]);
                let dst = StackRef::sp(dst, &func.stack_slots);
                let base = stk_base(dst.base);
                put_op1(bits, rex2(base, src), sink);
                modrm_sib_disp32(src, sink);
                sib_noindex(base, sink);
                sink.put4(dst.offset as u32);
                return;
            }
        }
        // Recipe RexOp1regspill32
        86 => {
            if let InstructionData::RegSpill {
                opcode,
                src,
                dst,
                arg,
                ..
            } = *inst_data {
                divert.apply(inst_data);
                sink.trap(TrapCode::StackOverflow, func.srclocs[inst]);
                let dst = StackRef::sp(dst, &func.stack_slots);
                let base = stk_base(dst.base);
                put_rexop1(bits, rex2(base, src), sink);
                modrm_sib_disp32(src, sink);
                sib_noindex(base, sink);
                sink.put4(dst.offset as u32);
                return;
            }
        }
        // Recipe Op1ld
        87 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op1(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else if needs_offset(in_reg0) {
                    modrm_disp8(in_reg0, out_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_rm(in_reg0, out_reg0, sink);
                }
                return;
            }
        }
        // Recipe RexOp1ld
        88 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop1(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else if needs_offset(in_reg0) {
                    modrm_disp8(in_reg0, out_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_rm(in_reg0, out_reg0, sink);
                }
                return;
            }
        }
        // Recipe Op2ld
        89 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op2(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else if needs_offset(in_reg0) {
                    modrm_disp8(in_reg0, out_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_rm(in_reg0, out_reg0, sink);
                }
                return;
            }
        }
        // Recipe RexOp2ld
        90 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop2(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else if needs_offset(in_reg0) {
                    modrm_disp8(in_reg0, out_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_rm(in_reg0, out_reg0, sink);
                }
                return;
            }
        }
        // Recipe Op1ldDisp8
        91 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op1(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib_disp8(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else {
                    modrm_disp8(in_reg0, out_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe RexOp1ldDisp8
        92 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop1(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib_disp8(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else {
                    modrm_disp8(in_reg0, out_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe Op2ldDisp8
        93 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op2(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib_disp8(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else {
                    modrm_disp8(in_reg0, out_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe RexOp2ldDisp8
        94 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop2(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib_disp8(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else {
                    modrm_disp8(in_reg0, out_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe Op1ldDisp32
        95 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op1(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib_disp32(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else {
                    modrm_disp32(in_reg0, out_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe RexOp1ldDisp32
        96 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop1(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib_disp32(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else {
                    modrm_disp32(in_reg0, out_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe Op2ldDisp32
        97 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op2(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib_disp32(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else {
                    modrm_disp32(in_reg0, out_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe RexOp2ldDisp32
        98 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop2(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib_disp32(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else {
                    modrm_disp32(in_reg0, out_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe Op1fillSib32
        99 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_stk0 = StackRef::masked(
                    divert.stack(args[0], &func.locations),
                    StackBaseMask(1),
                    &func.stack_slots,
                ).unwrap();
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                let base = stk_base(in_stk0.base);
                put_op1(bits, rex2(base, out_reg0), sink);
                modrm_sib_disp32(out_reg0, sink);
                sib_noindex(base, sink);
                sink.put4(in_stk0.offset as u32);
                return;
            }
        }
        // Recipe RexOp1fillSib32
        100 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_stk0 = StackRef::masked(
                    divert.stack(args[0], &func.locations),
                    StackBaseMask(1),
                    &func.stack_slots,
                ).unwrap();
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                let base = stk_base(in_stk0.base);
                put_rexop1(bits, rex2(base, out_reg0), sink);
                modrm_sib_disp32(out_reg0, sink);
                sib_noindex(base, sink);
                sink.put4(in_stk0.offset as u32);
                return;
            }
        }
        // Recipe Op1regfill32
        101 => {
            if let InstructionData::RegFill {
                opcode,
                src,
                dst,
                arg,
                ..
            } = *inst_data {
                divert.apply(inst_data);
                let src = StackRef::sp(src, &func.stack_slots);
                let base = stk_base(src.base);
                put_op1(bits, rex2(base, dst), sink);
                modrm_sib_disp32(dst, sink);
                sib_noindex(base, sink);
                sink.put4(src.offset as u32);
                return;
            }
        }
        // Recipe RexOp1regfill32
        102 => {
            if let InstructionData::RegFill {
                opcode,
                src,
                dst,
                arg,
                ..
            } = *inst_data {
                divert.apply(inst_data);
                let src = StackRef::sp(src, &func.stack_slots);
                let base = stk_base(src.base);
                put_rexop1(bits, rex2(base, dst), sink);
                modrm_sib_disp32(dst, sink);
                sib_noindex(base, sink);
                sink.put4(src.offset as u32);
                return;
            }
        }
        // Recipe fillnull
        103 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_stk0 = StackRef::masked(
                    divert.stack(args[0], &func.locations),
                    StackBaseMask(1),
                    &func.stack_slots,
                ).unwrap();
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                return;
            }
        }
        // Recipe ffillnull
        104 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_stk0 = StackRef::masked(
                    divert.stack(args[0], &func.locations),
                    StackBaseMask(1),
                    &func.stack_slots,
                ).unwrap();
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                return;
            }
        }
        // Recipe Op1pushq
        105 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                sink.trap(TrapCode::StackOverflow, func.srclocs[inst]);
                put_op1(bits | (in_reg0 & 7), rex1(in_reg0), sink);
                return;
            }
        }
        // Recipe RexOp1pushq
        106 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                sink.trap(TrapCode::StackOverflow, func.srclocs[inst]);
                put_rexop1(bits | (in_reg0 & 7), rex1(in_reg0), sink);
                return;
            }
        }
        // Recipe Op1popq
        107 => {
            if let InstructionData::NullAry {
                opcode,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_op1(bits | (out_reg0 & 7), rex1(out_reg0), sink);
                return;
            }
        }
        // Recipe RexOp1popq
        108 => {
            if let InstructionData::NullAry {
                opcode,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop1(bits | (out_reg0 & 7), rex1(out_reg0), sink);
                return;
            }
        }
        // Recipe RexOp1copysp
        109 => {
            if let InstructionData::CopySpecial {
                opcode,
                src,
                dst,
                ..
            } = *inst_data {
                put_rexop1(bits, rex2(dst, src), sink);
                modrm_rr(dst, src, sink);
                return;
            }
        }
        // Recipe Op1copysp
        110 => {
            if let InstructionData::CopySpecial {
                opcode,
                src,
                dst,
                ..
            } = *inst_data {
                put_op1(bits, rex2(dst, src), sink);
                modrm_rr(dst, src, sink);
                return;
            }
        }
        // Recipe Op1umr_reg_to_ssa
        111 => {
            if let InstructionData::CopyToSsa {
                opcode,
                src,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_op1(bits, rex2(out_reg0, src), sink);
                modrm_rr(out_reg0, src, sink);
                return;
            }
        }
        // Recipe RexOp1umr_reg_to_ssa
        112 => {
            if let InstructionData::CopyToSsa {
                opcode,
                src,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop1(bits, rex2(out_reg0, src), sink);
                modrm_rr(out_reg0, src, sink);
                return;
            }
        }
        // Recipe Mp2furm_reg_to_ssa
        113 => {
            if let InstructionData::CopyToSsa {
                opcode,
                src,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_mp2(bits, rex2(src, out_reg0), sink);
                modrm_rr(src, out_reg0, sink);
                return;
            }
        }
        // Recipe RexMp2furm_reg_to_ssa
        114 => {
            if let InstructionData::CopyToSsa {
                opcode,
                src,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexmp2(bits, rex2(src, out_reg0), sink);
                modrm_rr(src, out_reg0, sink);
                return;
            }
        }
        // Recipe stacknull
        115 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_stk0 = StackRef::masked(
                    divert.stack(args[0], &func.locations),
                    StackBaseMask(1),
                    &func.stack_slots,
                ).unwrap();
                let results = [func.dfg.first_result(inst)];
                let out_stk0 = StackRef::masked(
                    divert.stack(results[0], &func.locations),
                    StackBaseMask(1),
                    &func.stack_slots,
                ).unwrap();
                return;
            }
        }
        // Recipe Op1adjustsp
        116 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                put_op1(bits, rex2(RU::rsp.into(), in_reg0), sink);
                modrm_rr(RU::rsp.into(), in_reg0, sink);
                return;
            }
        }
        // Recipe RexOp1adjustsp
        117 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                put_rexop1(bits, rex2(RU::rsp.into(), in_reg0), sink);
                modrm_rr(RU::rsp.into(), in_reg0, sink);
                return;
            }
        }
        // Recipe Op1adjustsp_ib
        118 => {
            if let InstructionData::UnaryImm {
                opcode,
                imm,
                ..
            } = *inst_data {
                put_op1(bits, rex1(RU::rsp.into()), sink);
                modrm_r_bits(RU::rsp.into(), bits, sink);
                let imm: i64 = imm.into();
                sink.put1(imm as u8);
                return;
            }
        }
        // Recipe Op1adjustsp_id
        119 => {
            if let InstructionData::UnaryImm {
                opcode,
                imm,
                ..
            } = *inst_data {
                put_op1(bits, rex1(RU::rsp.into()), sink);
                modrm_r_bits(RU::rsp.into(), bits, sink);
                let imm: i64 = imm.into();
                sink.put4(imm as u32);
                return;
            }
        }
        // Recipe RexOp1adjustsp_ib
        120 => {
            if let InstructionData::UnaryImm {
                opcode,
                imm,
                ..
            } = *inst_data {
                put_rexop1(bits, rex1(RU::rsp.into()), sink);
                modrm_r_bits(RU::rsp.into(), bits, sink);
                let imm: i64 = imm.into();
                sink.put1(imm as u8);
                return;
            }
        }
        // Recipe RexOp1adjustsp_id
        121 => {
            if let InstructionData::UnaryImm {
                opcode,
                imm,
                ..
            } = *inst_data {
                put_rexop1(bits, rex1(RU::rsp.into()), sink);
                modrm_r_bits(RU::rsp.into(), bits, sink);
                let imm: i64 = imm.into();
                sink.put4(imm as u32);
                return;
            }
        }
        // Recipe Mp2fld
        122 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_mp2(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else if needs_offset(in_reg0) {
                    modrm_disp8(in_reg0, out_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_rm(in_reg0, out_reg0, sink);
                }
                return;
            }
        }
        // Recipe RexMp2fld
        123 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexmp2(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else if needs_offset(in_reg0) {
                    modrm_disp8(in_reg0, out_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_rm(in_reg0, out_reg0, sink);
                }
                return;
            }
        }
        // Recipe Mp2fldDisp8
        124 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_mp2(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib_disp8(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else {
                    modrm_disp8(in_reg0, out_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe RexMp2fldDisp8
        125 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexmp2(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib_disp8(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else {
                    modrm_disp8(in_reg0, out_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe Mp2fldDisp32
        126 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_mp2(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib_disp32(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else {
                    modrm_disp32(in_reg0, out_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe RexMp2fldDisp32
        127 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexmp2(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib_disp32(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else {
                    modrm_disp32(in_reg0, out_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe Mp2fldWithIndex
        128 => {
            if let InstructionData::LoadComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_mp2(bits, rex3(in_reg0, out_reg0, in_reg1), sink);
                // The else branch always inserts an SIB byte.
                if needs_offset(in_reg0) {
                    modrm_sib_disp8(out_reg0, sink);
                    sib(0, in_reg1, in_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_sib(out_reg0, sink);
                    sib(0, in_reg1, in_reg0, sink);
                }
                return;
            }
        }
        // Recipe RexMp2fldWithIndex
        129 => {
            if let InstructionData::LoadComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexmp2(bits, rex3(in_reg0, out_reg0, in_reg1), sink);
                // The else branch always inserts an SIB byte.
                if needs_offset(in_reg0) {
                    modrm_sib_disp8(out_reg0, sink);
                    sib(0, in_reg1, in_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_sib(out_reg0, sink);
                    sib(0, in_reg1, in_reg0, sink);
                }
                return;
            }
        }
        // Recipe Mp2fldWithIndexDisp8
        130 => {
            if let InstructionData::LoadComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_mp2(bits, rex3(in_reg0, out_reg0, in_reg1), sink);
                modrm_sib_disp8(out_reg0, sink);
                sib(0, in_reg1, in_reg0, sink);
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe RexMp2fldWithIndexDisp8
        131 => {
            if let InstructionData::LoadComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexmp2(bits, rex3(in_reg0, out_reg0, in_reg1), sink);
                modrm_sib_disp8(out_reg0, sink);
                sib(0, in_reg1, in_reg0, sink);
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe Mp2fldWithIndexDisp32
        132 => {
            if let InstructionData::LoadComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_mp2(bits, rex3(in_reg0, out_reg0, in_reg1), sink);
                modrm_sib_disp32(out_reg0, sink);
                sib(0, in_reg1, in_reg0, sink);
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe RexMp2fldWithIndexDisp32
        133 => {
            if let InstructionData::LoadComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexmp2(bits, rex3(in_reg0, out_reg0, in_reg1), sink);
                modrm_sib_disp32(out_reg0, sink);
                sib(0, in_reg1, in_reg0, sink);
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe Mp2fst
        134 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_mp2(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else if needs_offset(in_reg1) {
                    modrm_disp8(in_reg1, in_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_rm(in_reg1, in_reg0, sink);
                }
                return;
            }
        }
        // Recipe RexMp2fst
        135 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexmp2(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else if needs_offset(in_reg1) {
                    modrm_disp8(in_reg1, in_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_rm(in_reg1, in_reg0, sink);
                }
                return;
            }
        }
        // Recipe Mp2fstDisp8
        136 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_mp2(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib_disp8(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else {
                    modrm_disp8(in_reg1, in_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe RexMp2fstDisp8
        137 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexmp2(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib_disp8(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else {
                    modrm_disp8(in_reg1, in_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe Mp2fstDisp32
        138 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_mp2(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib_disp32(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else {
                    modrm_disp32(in_reg1, in_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe RexMp2fstDisp32
        139 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexmp2(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib_disp32(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else {
                    modrm_disp32(in_reg1, in_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe Mp2fstWithIndex
        140 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_mp2(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                // The else branch always inserts an SIB byte.
                if needs_offset(in_reg1) {
                    modrm_sib_disp8(in_reg0, sink);
                    sib(0, in_reg2, in_reg1, sink);
                    sink.put1(0);
                } else {
                    modrm_sib(in_reg0, sink);
                    sib(0, in_reg2, in_reg1, sink);
                }
                return;
            }
        }
        // Recipe RexMp2fstWithIndex
        141 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexmp2(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                // The else branch always inserts an SIB byte.
                if needs_offset(in_reg1) {
                    modrm_sib_disp8(in_reg0, sink);
                    sib(0, in_reg2, in_reg1, sink);
                    sink.put1(0);
                } else {
                    modrm_sib(in_reg0, sink);
                    sib(0, in_reg2, in_reg1, sink);
                }
                return;
            }
        }
        // Recipe Mp2fstWithIndexDisp8
        142 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_mp2(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                modrm_sib_disp8(in_reg0, sink);
                sib(0, in_reg2, in_reg1, sink);
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe RexMp2fstWithIndexDisp8
        143 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexmp2(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                modrm_sib_disp8(in_reg0, sink);
                sib(0, in_reg2, in_reg1, sink);
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe Mp2fstWithIndexDisp32
        144 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_mp2(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                modrm_sib_disp32(in_reg0, sink);
                sib(0, in_reg2, in_reg1, sink);
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe RexMp2fstWithIndexDisp32
        145 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexmp2(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                modrm_sib_disp32(in_reg0, sink);
                sib(0, in_reg2, in_reg1, sink);
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe Mp2ffillSib32
        146 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_stk0 = StackRef::masked(
                    divert.stack(args[0], &func.locations),
                    StackBaseMask(1),
                    &func.stack_slots,
                ).unwrap();
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                let base = stk_base(in_stk0.base);
                put_mp2(bits, rex2(base, out_reg0), sink);
                modrm_sib_disp32(out_reg0, sink);
                sib_noindex(base, sink);
                sink.put4(in_stk0.offset as u32);
                return;
            }
        }
        // Recipe RexMp2ffillSib32
        147 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_stk0 = StackRef::masked(
                    divert.stack(args[0], &func.locations),
                    StackBaseMask(1),
                    &func.stack_slots,
                ).unwrap();
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                let base = stk_base(in_stk0.base);
                put_rexmp2(bits, rex2(base, out_reg0), sink);
                modrm_sib_disp32(out_reg0, sink);
                sib_noindex(base, sink);
                sink.put4(in_stk0.offset as u32);
                return;
            }
        }
        // Recipe Mp2fregfill32
        148 => {
            if let InstructionData::RegFill {
                opcode,
                src,
                dst,
                arg,
                ..
            } = *inst_data {
                divert.apply(inst_data);
                let src = StackRef::sp(src, &func.stack_slots);
                let base = stk_base(src.base);
                put_mp2(bits, rex2(base, dst), sink);
                modrm_sib_disp32(dst, sink);
                sib_noindex(base, sink);
                sink.put4(src.offset as u32);
                return;
            }
        }
        // Recipe RexMp2fregfill32
        149 => {
            if let InstructionData::RegFill {
                opcode,
                src,
                dst,
                arg,
                ..
            } = *inst_data {
                divert.apply(inst_data);
                let src = StackRef::sp(src, &func.stack_slots);
                let base = stk_base(src.base);
                put_rexmp2(bits, rex2(base, dst), sink);
                modrm_sib_disp32(dst, sink);
                sib_noindex(base, sink);
                sink.put4(src.offset as u32);
                return;
            }
        }
        // Recipe Mp2fspillSib32
        150 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_stk0 = StackRef::masked(
                    divert.stack(results[0], &func.locations),
                    StackBaseMask(1),
                    &func.stack_slots,
                ).unwrap();
                sink.trap(TrapCode::StackOverflow, func.srclocs[inst]);
                let base = stk_base(out_stk0.base);
                put_mp2(bits, rex2(base, in_reg0), sink);
                modrm_sib_disp32(in_reg0, sink);
                sib_noindex(base, sink);
                sink.put4(out_stk0.offset as u32);
                return;
            }
        }
        // Recipe RexMp2fspillSib32
        151 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_stk0 = StackRef::masked(
                    divert.stack(results[0], &func.locations),
                    StackBaseMask(1),
                    &func.stack_slots,
                ).unwrap();
                sink.trap(TrapCode::StackOverflow, func.srclocs[inst]);
                let base = stk_base(out_stk0.base);
                put_rexmp2(bits, rex2(base, in_reg0), sink);
                modrm_sib_disp32(in_reg0, sink);
                sib_noindex(base, sink);
                sink.put4(out_stk0.offset as u32);
                return;
            }
        }
        // Recipe Mp2fregspill32
        152 => {
            if let InstructionData::RegSpill {
                opcode,
                src,
                dst,
                arg,
                ..
            } = *inst_data {
                divert.apply(inst_data);
                sink.trap(TrapCode::StackOverflow, func.srclocs[inst]);
                let dst = StackRef::sp(dst, &func.stack_slots);
                let base = stk_base(dst.base);
                put_mp2(bits, rex2(base, src), sink);
                modrm_sib_disp32(src, sink);
                sib_noindex(base, sink);
                sink.put4(dst.offset as u32);
                return;
            }
        }
        // Recipe RexMp2fregspill32
        153 => {
            if let InstructionData::RegSpill {
                opcode,
                src,
                dst,
                arg,
                ..
            } = *inst_data {
                divert.apply(inst_data);
                sink.trap(TrapCode::StackOverflow, func.srclocs[inst]);
                let dst = StackRef::sp(dst, &func.stack_slots);
                let base = stk_base(dst.base);
                put_rexmp2(bits, rex2(base, src), sink);
                modrm_sib_disp32(src, sink);
                sib_noindex(base, sink);
                sink.put4(dst.offset as u32);
                return;
            }
        }
        // Recipe Op1fnaddr4
        154 => {
            if let InstructionData::FuncAddr {
                opcode,
                func_ref,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_op1(bits | (out_reg0 & 7), rex1(out_reg0), sink);
                sink.reloc_external(Reloc::Abs4,
                                    &func.dfg.ext_funcs[func_ref].name,
                                    0);
                sink.put4(0);
                return;
            }
        }
        // Recipe RexOp1fnaddr8
        155 => {
            if let InstructionData::FuncAddr {
                opcode,
                func_ref,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop1(bits | (out_reg0 & 7), rex1(out_reg0), sink);
                sink.reloc_external(Reloc::Abs8,
                                    &func.dfg.ext_funcs[func_ref].name,
                                    0);
                sink.put8(0);
                return;
            }
        }
        // Recipe Op1allones_fnaddr4
        156 => {
            if let InstructionData::FuncAddr {
                opcode,
                func_ref,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_op1(bits | (out_reg0 & 7), rex1(out_reg0), sink);
                sink.reloc_external(Reloc::Abs4,
                                    &func.dfg.ext_funcs[func_ref].name,
                                    0);
                // Write the immediate as `!0` for the benefit of BaldrMonkey.
                sink.put4(!0);
                return;
            }
        }
        // Recipe RexOp1allones_fnaddr8
        157 => {
            if let InstructionData::FuncAddr {
                opcode,
                func_ref,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop1(bits | (out_reg0 & 7), rex1(out_reg0), sink);
                sink.reloc_external(Reloc::Abs8,
                                    &func.dfg.ext_funcs[func_ref].name,
                                    0);
                // Write the immediate as `!0` for the benefit of BaldrMonkey.
                sink.put8(!0);
                return;
            }
        }
        // Recipe RexOp1pcrel_fnaddr8
        158 => {
            if let InstructionData::FuncAddr {
                opcode,
                func_ref,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop1(bits, rex2(0, out_reg0), sink);
                modrm_riprel(out_reg0, sink);
                // The addend adjusts for the difference between the end of the
                // instruction and the beginning of the immediate field.
                sink.reloc_external(Reloc::X86PCRel4,
                                    &func.dfg.ext_funcs[func_ref].name,
                                    -4);
                sink.put4(0);
                return;
            }
        }
        // Recipe RexOp1got_fnaddr8
        159 => {
            if let InstructionData::FuncAddr {
                opcode,
                func_ref,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop1(bits, rex2(0, out_reg0), sink);
                modrm_riprel(out_reg0, sink);
                // The addend adjusts for the difference between the end of the
                // instruction and the beginning of the immediate field.
                sink.reloc_external(Reloc::X86GOTPCRel4,
                                    &func.dfg.ext_funcs[func_ref].name,
                                    -4);
                sink.put4(0);
                return;
            }
        }
        // Recipe Op1gvaddr4
        160 => {
            if let InstructionData::UnaryGlobalValue {
                opcode,
                global_value,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_op1(bits | (out_reg0 & 7), rex1(out_reg0), sink);
                sink.reloc_external(Reloc::Abs4,
                                    &func.global_values[global_value].symbol_name(),
                                    0);
                sink.put4(0);
                return;
            }
        }
        // Recipe RexOp1gvaddr8
        161 => {
            if let InstructionData::UnaryGlobalValue {
                opcode,
                global_value,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop1(bits | (out_reg0 & 7), rex1(out_reg0), sink);
                sink.reloc_external(Reloc::Abs8,
                                    &func.global_values[global_value].symbol_name(),
                                    0);
                sink.put8(0);
                return;
            }
        }
        // Recipe RexOp1pcrel_gvaddr8
        162 => {
            if let InstructionData::UnaryGlobalValue {
                opcode,
                global_value,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop1(bits, rex2(0, out_reg0), sink);
                modrm_rm(5, out_reg0, sink);
                // The addend adjusts for the difference between the end of the
                // instruction and the beginning of the immediate field.
                sink.reloc_external(Reloc::X86PCRel4,
                                    &func.global_values[global_value].symbol_name(),
                                    -4);
                sink.put4(0);
                return;
            }
        }
        // Recipe RexOp1got_gvaddr8
        163 => {
            if let InstructionData::UnaryGlobalValue {
                opcode,
                global_value,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop1(bits, rex2(0, out_reg0), sink);
                modrm_rm(5, out_reg0, sink);
                // The addend adjusts for the difference between the end of the
                // instruction and the beginning of the immediate field.
                sink.reloc_external(Reloc::X86GOTPCRel4,
                                    &func.global_values[global_value].symbol_name(),
                                    -4);
                sink.put4(0);
                return;
            }
        }
        // Recipe Op1spaddr4_id
        164 => {
            if let InstructionData::StackLoad {
                opcode,
                stack_slot,
                offset,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                let sp = StackRef::sp(stack_slot, &func.stack_slots);
                let base = stk_base(sp.base);
                put_op1(bits, rex2(out_reg0, base), sink);
                modrm_sib_disp8(out_reg0, sink);
                sib_noindex(base, sink);
                let imm : i32 = offset.into();
                sink.put4(sp.offset.checked_add(imm).unwrap() as u32);
                return;
            }
        }
        // Recipe RexOp1spaddr8_id
        165 => {
            if let InstructionData::StackLoad {
                opcode,
                stack_slot,
                offset,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                let sp = StackRef::sp(stack_slot, &func.stack_slots);
                let base = stk_base(sp.base);
                put_rexop1(bits, rex2(base, out_reg0), sink);
                modrm_sib_disp32(out_reg0, sink);
                sib_noindex(base, sink);
                let imm : i32 = offset.into();
                sink.put4(sp.offset.checked_add(imm).unwrap() as u32);
                return;
            }
        }
        // Recipe Op1call_id
        166 => {
            if let InstructionData::Call {
                opcode,
                func_ref,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                sink.trap(TrapCode::StackOverflow, func.srclocs[inst]);
                put_op1(bits, BASE_REX, sink);
                // The addend adjusts for the difference between the end of the
                // instruction and the beginning of the immediate field.
                sink.reloc_external(Reloc::X86CallPCRel4,
                                    &func.dfg.ext_funcs[func_ref].name,
                                    -4);
                sink.put4(0);
                return;
            }
        }
        // Recipe Op1call_plt_id
        167 => {
            if let InstructionData::Call {
                opcode,
                func_ref,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                sink.trap(TrapCode::StackOverflow, func.srclocs[inst]);
                put_op1(bits, BASE_REX, sink);
                sink.reloc_external(Reloc::X86CallPLTRel4,
                                    &func.dfg.ext_funcs[func_ref].name,
                                    -4);
                sink.put4(0);
                return;
            }
        }
        // Recipe Op1call_r
        168 => {
            if let InstructionData::CallIndirect {
                opcode,
                sig_ref,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                sink.trap(TrapCode::StackOverflow, func.srclocs[inst]);
                put_op1(bits, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                return;
            }
        }
        // Recipe RexOp1call_r
        169 => {
            if let InstructionData::CallIndirect {
                opcode,
                sig_ref,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                sink.trap(TrapCode::StackOverflow, func.srclocs[inst]);
                put_rexop1(bits, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                return;
            }
        }
        // Recipe Op1ret
        170 => {
            if let InstructionData::MultiAry {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                put_op1(bits, BASE_REX, sink);
                return;
            }
        }
        // Recipe Op1jmpb
        171 => {
            if let InstructionData::Jump {
                opcode,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                put_op1(bits, BASE_REX, sink);
                disp1(destination, func, sink);
                return;
            }
        }
        // Recipe Op1jmpd
        172 => {
            if let InstructionData::Jump {
                opcode,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                put_op1(bits, BASE_REX, sink);
                disp4(destination, func, sink);
                return;
            }
        }
        // Recipe Op1brib
        173 => {
            if let InstructionData::BranchInt {
                opcode,
                cond,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                put_op1(bits | icc2opc(cond), BASE_REX, sink);
                disp1(destination, func, sink);
                return;
            }
        }
        // Recipe RexOp1brib
        174 => {
            if let InstructionData::BranchInt {
                opcode,
                cond,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                put_rexop1(bits | icc2opc(cond), BASE_REX, sink);
                disp1(destination, func, sink);
                return;
            }
        }
        // Recipe Op2brid
        175 => {
            if let InstructionData::BranchInt {
                opcode,
                cond,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                put_op2(bits | icc2opc(cond), BASE_REX, sink);
                disp4(destination, func, sink);
                return;
            }
        }
        // Recipe RexOp2brid
        176 => {
            if let InstructionData::BranchInt {
                opcode,
                cond,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                put_rexop2(bits | icc2opc(cond), BASE_REX, sink);
                disp4(destination, func, sink);
                return;
            }
        }
        // Recipe Op1brfb
        177 => {
            if let InstructionData::BranchFloat {
                opcode,
                cond,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                put_op1(bits | fcc2opc(cond), BASE_REX, sink);
                disp1(destination, func, sink);
                return;
            }
        }
        // Recipe RexOp1brfb
        178 => {
            if let InstructionData::BranchFloat {
                opcode,
                cond,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                put_rexop1(bits | fcc2opc(cond), BASE_REX, sink);
                disp1(destination, func, sink);
                return;
            }
        }
        // Recipe Op2brfd
        179 => {
            if let InstructionData::BranchFloat {
                opcode,
                cond,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                put_op2(bits | fcc2opc(cond), BASE_REX, sink);
                disp4(destination, func, sink);
                return;
            }
        }
        // Recipe RexOp2brfd
        180 => {
            if let InstructionData::BranchFloat {
                opcode,
                cond,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                put_rexop2(bits | fcc2opc(cond), BASE_REX, sink);
                disp4(destination, func, sink);
                return;
            }
        }
        // Recipe Op1tjccb
        181 => {
            if let InstructionData::Branch {
                opcode,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                // test r, r.
                put_op1((bits & 0xff00) | 0x85, rex2(in_reg0, in_reg0), sink);
                modrm_rr(in_reg0, in_reg0, sink);
                // Jcc instruction.
                sink.put1(bits as u8);
                disp1(destination, func, sink);
                return;
            }
        }
        // Recipe RexOp1tjccb
        182 => {
            if let InstructionData::Branch {
                opcode,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                // test r, r.
                put_rexop1((bits & 0xff00) | 0x85, rex2(in_reg0, in_reg0), sink);
                modrm_rr(in_reg0, in_reg0, sink);
                // Jcc instruction.
                sink.put1(bits as u8);
                disp1(destination, func, sink);
                return;
            }
        }
        // Recipe Op1tjccd
        183 => {
            if let InstructionData::Branch {
                opcode,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                // test r, r.
                put_op1((bits & 0xff00) | 0x85, rex2(in_reg0, in_reg0), sink);
                modrm_rr(in_reg0, in_reg0, sink);
                // Jcc instruction.
                sink.put1(0x0f);
                sink.put1(bits as u8);
                disp4(destination, func, sink);
                return;
            }
        }
        // Recipe RexOp1tjccd
        184 => {
            if let InstructionData::Branch {
                opcode,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                // test r, r.
                put_rexop1((bits & 0xff00) | 0x85, rex2(in_reg0, in_reg0), sink);
                modrm_rr(in_reg0, in_reg0, sink);
                // Jcc instruction.
                sink.put1(0x0f);
                sink.put1(bits as u8);
                disp4(destination, func, sink);
                return;
            }
        }
        // Recipe Op1t8jccd_long
        185 => {
            if let InstructionData::Branch {
                opcode,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                // test32 r, 0xff.
                put_op1((bits & 0xff00) | 0xf7, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                sink.put4(0xff);
                // Jcc instruction.
                sink.put1(0x0f);
                sink.put1(bits as u8);
                disp4(destination, func, sink);
                return;
            }
        }
        // Recipe Op1t8jccb_abcd
        186 => {
            if let InstructionData::Branch {
                opcode,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                // test8 r, r.
                put_op1((bits & 0xff00) | 0x84, rex2(in_reg0, in_reg0), sink);
                modrm_rr(in_reg0, in_reg0, sink);
                // Jcc instruction.
                sink.put1(bits as u8);
                disp1(destination, func, sink);
                return;
            }
        }
        // Recipe RexOp1t8jccb
        187 => {
            if let InstructionData::Branch {
                opcode,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                // test8 r, r.
                put_rexop1((bits & 0xff00) | 0x84, rex2(in_reg0, in_reg0), sink);
                modrm_rr(in_reg0, in_reg0, sink);
                // Jcc instruction.
                sink.put1(bits as u8);
                disp1(destination, func, sink);
                return;
            }
        }
        // Recipe Op1t8jccd_abcd
        188 => {
            if let InstructionData::Branch {
                opcode,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                // test8 r, r.
                put_op1((bits & 0xff00) | 0x84, rex2(in_reg0, in_reg0), sink);
                modrm_rr(in_reg0, in_reg0, sink);
                // Jcc instruction.
                sink.put1(0x0f);
                sink.put1(bits as u8);
                disp4(destination, func, sink);
                return;
            }
        }
        // Recipe RexOp1t8jccd
        189 => {
            if let InstructionData::Branch {
                opcode,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                // test8 r, r.
                put_rexop1((bits & 0xff00) | 0x84, rex2(in_reg0, in_reg0), sink);
                modrm_rr(in_reg0, in_reg0, sink);
                // Jcc instruction.
                sink.put1(0x0f);
                sink.put1(bits as u8);
                disp4(destination, func, sink);
                return;
            }
        }
        // Recipe RexOp1jt_entry
        190 => {
            if let InstructionData::BranchTableEntry {
                opcode,
                imm,
                table,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop1(bits, rex3(in_reg1, out_reg0, in_reg0), sink);
                if needs_offset(in_reg1) {
                    modrm_sib_disp8(out_reg0, sink);
                    sib(imm.trailing_zeros() as u8, in_reg0, in_reg1, sink);
                    sink.put1(0);
                } else {
                    modrm_sib(out_reg0, sink);
                    sib(imm.trailing_zeros() as u8, in_reg0, in_reg1, sink);
                }
                return;
            }
        }
        // Recipe Op1jt_entry
        191 => {
            if let InstructionData::BranchTableEntry {
                opcode,
                imm,
                table,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_op1(bits, rex3(in_reg1, out_reg0, in_reg0), sink);
                if needs_offset(in_reg1) {
                    modrm_sib_disp8(out_reg0, sink);
                    sib(imm.trailing_zeros() as u8, in_reg0, in_reg1, sink);
                    sink.put1(0);
                } else {
                    modrm_sib(out_reg0, sink);
                    sib(imm.trailing_zeros() as u8, in_reg0, in_reg1, sink);
                }
                return;
            }
        }
        // Recipe RexOp1jt_base
        192 => {
            if let InstructionData::BranchTableBase {
                opcode,
                table,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop1(bits, rex2(0, out_reg0), sink);
                modrm_riprel(out_reg0, sink);
                
                // No reloc is needed here as the jump table is emitted directly after
                // the function body.
                jt_disp4(table, func, sink);
                return;
            }
        }
        // Recipe Op1jt_base
        193 => {
            if let InstructionData::BranchTableBase {
                opcode,
                table,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_op1(bits, rex2(0, out_reg0), sink);
                modrm_riprel(out_reg0, sink);
                
                // No reloc is needed here as the jump table is emitted directly after
                // the function body.
                jt_disp4(table, func, sink);
                return;
            }
        }
        // Recipe RexOp1indirect_jmp
        194 => {
            if let InstructionData::IndirectJump {
                opcode,
                table,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                put_rexop1(bits, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                return;
            }
        }
        // Recipe Op1indirect_jmp
        195 => {
            if let InstructionData::IndirectJump {
                opcode,
                table,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                put_op1(bits, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                return;
            }
        }
        // Recipe Op2trap
        196 => {
            if let InstructionData::Trap {
                opcode,
                code,
                ..
            } = *inst_data {
                sink.trap(code, func.srclocs[inst]);
                put_op2(bits, BASE_REX, sink);
                return;
            }
        }
        // Recipe debugtrap
        197 => {
            if let InstructionData::NullAry {
                opcode,
                ..
            } = *inst_data {
                sink.put1(0xcc);
                return;
            }
        }
        // Recipe trapif
        198 => {
            if let InstructionData::IntCondTrap {
                opcode,
                cond,
                code,
                ..
            } = *inst_data {
                // Jump over a 2-byte ud2.
                sink.put1(0x70 | (icc2opc(cond.inverse()) as u8));
                sink.put1(2);
                // ud2.
                sink.trap(code, func.srclocs[inst]);
                sink.put1(0x0f);
                sink.put1(0x0b);
                return;
            }
        }
        // Recipe trapff
        199 => {
            if let InstructionData::FloatCondTrap {
                opcode,
                cond,
                code,
                ..
            } = *inst_data {
                // Jump over a 2-byte ud2.
                sink.put1(0x70 | (fcc2opc(cond.inverse()) as u8));
                sink.put1(2);
                // ud2.
                sink.trap(code, func.srclocs[inst]);
                sink.put1(0x0f);
                sink.put1(0x0b);
                return;
            }
        }
        // Recipe Op1icscc
        200 => {
            if let InstructionData::IntCompare {
                opcode,
                cond,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                // Comparison instruction.
                put_op1(bits, rex2(in_reg0, in_reg1), sink);
                modrm_rr(in_reg0, in_reg1, sink);
                // `setCC` instruction, no REX.
                let setcc = 0x90 | icc2opc(cond);
                sink.put1(0x0f);
                sink.put1(setcc as u8);
                modrm_rr(out_reg0, 0, sink);
                return;
            }
        }
        // Recipe RexOp1icscc
        201 => {
            if let InstructionData::IntCompare {
                opcode,
                cond,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                // Comparison instruction.
                put_rexop1(bits, rex2(in_reg0, in_reg1), sink);
                modrm_rr(in_reg0, in_reg1, sink);
                // `setCC` instruction, no REX.
                let setcc = 0x90 | icc2opc(cond);
                sink.put1(0x0f);
                sink.put1(setcc as u8);
                modrm_rr(out_reg0, 0, sink);
                return;
            }
        }
        // Recipe Op1icscc_ib
        202 => {
            if let InstructionData::IntCompareImm {
                opcode,
                cond,
                imm,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                // Comparison instruction.
                put_op1(bits, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                let imm: i64 = imm.into();
                sink.put1(imm as u8);
                // `setCC` instruction, no REX.
                let setcc = 0x90 | icc2opc(cond);
                sink.put1(0x0f);
                sink.put1(setcc as u8);
                modrm_rr(out_reg0, 0, sink);
                return;
            }
        }
        // Recipe RexOp1icscc_ib
        203 => {
            if let InstructionData::IntCompareImm {
                opcode,
                cond,
                imm,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                // Comparison instruction.
                put_rexop1(bits, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                let imm: i64 = imm.into();
                sink.put1(imm as u8);
                // `setCC` instruction, no REX.
                let setcc = 0x90 | icc2opc(cond);
                sink.put1(0x0f);
                sink.put1(setcc as u8);
                modrm_rr(out_reg0, 0, sink);
                return;
            }
        }
        // Recipe Op1icscc_id
        204 => {
            if let InstructionData::IntCompareImm {
                opcode,
                cond,
                imm,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                // Comparison instruction.
                put_op1(bits, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                let imm: i64 = imm.into();
                sink.put4(imm as u32);
                // `setCC` instruction, no REX.
                let setcc = 0x90 | icc2opc(cond);
                sink.put1(0x0f);
                sink.put1(setcc as u8);
                modrm_rr(out_reg0, 0, sink);
                return;
            }
        }
        // Recipe RexOp1icscc_id
        205 => {
            if let InstructionData::IntCompareImm {
                opcode,
                cond,
                imm,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                // Comparison instruction.
                put_rexop1(bits, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                let imm: i64 = imm.into();
                sink.put4(imm as u32);
                // `setCC` instruction, no REX.
                let setcc = 0x90 | icc2opc(cond);
                sink.put1(0x0f);
                sink.put1(setcc as u8);
                modrm_rr(out_reg0, 0, sink);
                return;
            }
        }
        // Recipe Op1rcmp
        206 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_op1(bits, rex2(in_reg0, in_reg1), sink);
                modrm_rr(in_reg0, in_reg1, sink);
                return;
            }
        }
        // Recipe RexOp1rcmp
        207 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_rexop1(bits, rex2(in_reg0, in_reg1), sink);
                modrm_rr(in_reg0, in_reg1, sink);
                return;
            }
        }
        // Recipe Op1rcmp_ib
        208 => {
            if let InstructionData::BinaryImm {
                opcode,
                imm,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                put_op1(bits, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                let imm: i64 = imm.into();
                sink.put1(imm as u8);
                return;
            }
        }
        // Recipe RexOp1rcmp_ib
        209 => {
            if let InstructionData::BinaryImm {
                opcode,
                imm,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                put_rexop1(bits, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                let imm: i64 = imm.into();
                sink.put1(imm as u8);
                return;
            }
        }
        // Recipe Op1rcmp_id
        210 => {
            if let InstructionData::BinaryImm {
                opcode,
                imm,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                put_op1(bits, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                let imm: i64 = imm.into();
                sink.put4(imm as u32);
                return;
            }
        }
        // Recipe RexOp1rcmp_id
        211 => {
            if let InstructionData::BinaryImm {
                opcode,
                imm,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                put_rexop1(bits, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                let imm: i64 = imm.into();
                sink.put4(imm as u32);
                return;
            }
        }
        // Recipe Op1rcmp_sp
        212 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                put_op1(bits, rex2(in_reg0, RU::rsp.into()), sink);
                modrm_rr(in_reg0, RU::rsp.into(), sink);
                return;
            }
        }
        // Recipe RexOp1rcmp_sp
        213 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                put_rexop1(bits, rex2(in_reg0, RU::rsp.into()), sink);
                modrm_rr(in_reg0, RU::rsp.into(), sink);
                return;
            }
        }
        // Recipe Op2seti_abcd
        214 => {
            if let InstructionData::IntCond {
                opcode,
                cond,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_op2(bits | icc2opc(cond), rex1(out_reg0), sink);
                modrm_r_bits(out_reg0, bits, sink);
                return;
            }
        }
        // Recipe RexOp2seti
        215 => {
            if let InstructionData::IntCond {
                opcode,
                cond,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop2(bits | icc2opc(cond), rex1(out_reg0), sink);
                modrm_r_bits(out_reg0, bits, sink);
                return;
            }
        }
        // Recipe Op2setf_abcd
        216 => {
            if let InstructionData::FloatCond {
                opcode,
                cond,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_op2(bits | fcc2opc(cond), rex1(out_reg0), sink);
                modrm_r_bits(out_reg0, bits, sink);
                return;
            }
        }
        // Recipe RexOp2setf
        217 => {
            if let InstructionData::FloatCond {
                opcode,
                cond,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop2(bits | fcc2opc(cond), rex1(out_reg0), sink);
                modrm_r_bits(out_reg0, bits, sink);
                return;
            }
        }
        // Recipe Op2cmov
        218 => {
            if let InstructionData::IntSelect {
                opcode,
                cond,
                ref args,
                ..
            } = *inst_data {
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                put_op2(bits | icc2opc(cond), rex2(in_reg1, in_reg2), sink);
                modrm_rr(in_reg1, in_reg2, sink);
                return;
            }
        }
        // Recipe RexOp2cmov
        219 => {
            if let InstructionData::IntSelect {
                opcode,
                cond,
                ref args,
                ..
            } = *inst_data {
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                put_rexop2(bits | icc2opc(cond), rex2(in_reg1, in_reg2), sink);
                modrm_rr(in_reg1, in_reg2, sink);
                return;
            }
        }
        // Recipe Op2bsf_and_bsr
        220 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = func.dfg.inst_results(inst);
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_op2(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe RexOp2bsf_and_bsr
        221 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = func.dfg.inst_results(inst);
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop2(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe RexOp2urm_noflags
        222 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop2(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe Op2urm_noflags_abcd
        223 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_op2(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe null
        224 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                return;
            }
        }
        // Recipe Op2urm_noflags
        225 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_op2(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe RexOp1urm_noflags
        226 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop1(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe Op2f32imm_z
        227 => {
            if let InstructionData::UnaryIeee32 {
                opcode,
                imm,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_op2(bits, rex2(out_reg0, out_reg0), sink);
                modrm_rr(out_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe Mp2f64imm_z
        228 => {
            if let InstructionData::UnaryIeee64 {
                opcode,
                imm,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_mp2(bits, rex2(out_reg0, out_reg0), sink);
                modrm_rr(out_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe RexOp2f32imm_z
        229 => {
            if let InstructionData::UnaryIeee32 {
                opcode,
                imm,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop2(bits, rex2(out_reg0, out_reg0), sink);
                modrm_rr(out_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe RexMp2f64imm_z
        230 => {
            if let InstructionData::UnaryIeee64 {
                opcode,
                imm,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexmp2(bits, rex2(out_reg0, out_reg0), sink);
                modrm_rr(out_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe Mp2frurm
        231 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_mp2(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe RexMp2frurm
        232 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexmp2(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe Mp2rfumr
        233 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_mp2(bits, rex2(out_reg0, in_reg0), sink);
                modrm_rr(out_reg0, in_reg0, sink);
                return;
            }
        }
        // Recipe RexMp2rfumr
        234 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexmp2(bits, rex2(out_reg0, in_reg0), sink);
                modrm_rr(out_reg0, in_reg0, sink);
                return;
            }
        }
        // Recipe Op2furm
        235 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_op2(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe RexOp2furm
        236 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop2(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe Op2frmov
        237 => {
            if let InstructionData::RegMove {
                opcode,
                src,
                dst,
                arg,
                ..
            } = *inst_data {
                divert.apply(inst_data);
                put_op2(bits, rex2(src, dst), sink);
                modrm_rr(src, dst, sink);
                return;
            }
        }
        // Recipe RexOp2frmov
        238 => {
            if let InstructionData::RegMove {
                opcode,
                src,
                dst,
                arg,
                ..
            } = *inst_data {
                divert.apply(inst_data);
                put_rexop2(bits, rex2(src, dst), sink);
                modrm_rr(src, dst, sink);
                return;
            }
        }
        // Recipe Mp2furm
        239 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_mp2(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe RexMp2furm
        240 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexmp2(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe Mp2rfurm
        241 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_mp2(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe RexMp2rfurm
        242 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexmp2(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe Mp3furmi_rnd
        243 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_mp3(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                sink.put1(match opcode {
                    Opcode::Nearest => 0b00,
                    Opcode::Floor => 0b01,
                    Opcode::Ceil => 0b10,
                    Opcode::Trunc => 0b11,
                    x => panic!("{} unexpected for furmi_rnd", opcode),
                });
                return;
            }
        }
        // Recipe RexMp3furmi_rnd
        244 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexmp3(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                sink.put1(match opcode {
                    Opcode::Nearest => 0b00,
                    Opcode::Floor => 0b01,
                    Opcode::Ceil => 0b10,
                    Opcode::Trunc => 0b11,
                    x => panic!("{} unexpected for furmi_rnd", opcode),
                });
                return;
            }
        }
        // Recipe Mp2fa
        245 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_mp2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                return;
            }
        }
        // Recipe RexMp2fa
        246 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_rexmp2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                return;
            }
        }
        // Recipe Op2fa
        247 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_op2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                return;
            }
        }
        // Recipe RexOp2fa
        248 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_rexop2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                return;
            }
        }
        // Recipe Op2fax
        249 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_op2(bits, rex2(in_reg0, in_reg1), sink);
                modrm_rr(in_reg0, in_reg1, sink);
                return;
            }
        }
        // Recipe RexOp2fax
        250 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_rexop2(bits, rex2(in_reg0, in_reg1), sink);
                modrm_rr(in_reg0, in_reg1, sink);
                return;
            }
        }
        // Recipe Op2fcscc
        251 => {
            if let InstructionData::FloatCompare {
                opcode,
                cond,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                // Comparison instruction.
                put_op2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                // `setCC` instruction, no REX.
                use crate::ir::condcodes::FloatCC::*;
                let setcc = match cond {
                    Ordered                    => 0x9b, // EQ|LT|GT => setnp (P=0)
                    Unordered                  => 0x9a, // UN       => setp  (P=1)
                    OrderedNotEqual            => 0x95, // LT|GT    => setne (Z=0),
                    UnorderedOrEqual           => 0x94, // UN|EQ    => sete  (Z=1)
                    GreaterThan                => 0x97, // GT       => seta  (C=0&Z=0)
                    GreaterThanOrEqual         => 0x93, // GT|EQ    => setae (C=0)
                    UnorderedOrLessThan        => 0x92, // UN|LT    => setb  (C=1)
                    UnorderedOrLessThanOrEqual => 0x96, // UN|LT|EQ => setbe (Z=1|C=1)
                    Equal |                       // EQ
                    NotEqual |                    // UN|LT|GT
                    LessThan |                    // LT
                    LessThanOrEqual |             // LT|EQ
                    UnorderedOrGreaterThan |      // UN|GT
                    UnorderedOrGreaterThanOrEqual // UN|GT|EQ
                    => panic!("{} not supported by fcscc", cond),
                };
                sink.put1(0x0f);
                sink.put1(setcc);
                modrm_rr(out_reg0, 0, sink);
                return;
            }
        }
        // Recipe RexOp2fcscc
        252 => {
            if let InstructionData::FloatCompare {
                opcode,
                cond,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                // Comparison instruction.
                put_rexop2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                // `setCC` instruction, no REX.
                use crate::ir::condcodes::FloatCC::*;
                let setcc = match cond {
                    Ordered                    => 0x9b, // EQ|LT|GT => setnp (P=0)
                    Unordered                  => 0x9a, // UN       => setp  (P=1)
                    OrderedNotEqual            => 0x95, // LT|GT    => setne (Z=0),
                    UnorderedOrEqual           => 0x94, // UN|EQ    => sete  (Z=1)
                    GreaterThan                => 0x97, // GT       => seta  (C=0&Z=0)
                    GreaterThanOrEqual         => 0x93, // GT|EQ    => setae (C=0)
                    UnorderedOrLessThan        => 0x92, // UN|LT    => setb  (C=1)
                    UnorderedOrLessThanOrEqual => 0x96, // UN|LT|EQ => setbe (Z=1|C=1)
                    Equal |                       // EQ
                    NotEqual |                    // UN|LT|GT
                    LessThan |                    // LT
                    LessThanOrEqual |             // LT|EQ
                    UnorderedOrGreaterThan |      // UN|GT
                    UnorderedOrGreaterThanOrEqual // UN|GT|EQ
                    => panic!("{} not supported by fcscc", cond),
                };
                sink.put1(0x0f);
                sink.put1(setcc);
                modrm_rr(out_reg0, 0, sink);
                return;
            }
        }
        // Recipe Mp2fcscc
        253 => {
            if let InstructionData::FloatCompare {
                opcode,
                cond,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                // Comparison instruction.
                put_mp2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                // `setCC` instruction, no REX.
                use crate::ir::condcodes::FloatCC::*;
                let setcc = match cond {
                    Ordered                    => 0x9b, // EQ|LT|GT => setnp (P=0)
                    Unordered                  => 0x9a, // UN       => setp  (P=1)
                    OrderedNotEqual            => 0x95, // LT|GT    => setne (Z=0),
                    UnorderedOrEqual           => 0x94, // UN|EQ    => sete  (Z=1)
                    GreaterThan                => 0x97, // GT       => seta  (C=0&Z=0)
                    GreaterThanOrEqual         => 0x93, // GT|EQ    => setae (C=0)
                    UnorderedOrLessThan        => 0x92, // UN|LT    => setb  (C=1)
                    UnorderedOrLessThanOrEqual => 0x96, // UN|LT|EQ => setbe (Z=1|C=1)
                    Equal |                       // EQ
                    NotEqual |                    // UN|LT|GT
                    LessThan |                    // LT
                    LessThanOrEqual |             // LT|EQ
                    UnorderedOrGreaterThan |      // UN|GT
                    UnorderedOrGreaterThanOrEqual // UN|GT|EQ
                    => panic!("{} not supported by fcscc", cond),
                };
                sink.put1(0x0f);
                sink.put1(setcc);
                modrm_rr(out_reg0, 0, sink);
                return;
            }
        }
        // Recipe RexMp2fcscc
        254 => {
            if let InstructionData::FloatCompare {
                opcode,
                cond,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                // Comparison instruction.
                put_rexmp2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                // `setCC` instruction, no REX.
                use crate::ir::condcodes::FloatCC::*;
                let setcc = match cond {
                    Ordered                    => 0x9b, // EQ|LT|GT => setnp (P=0)
                    Unordered                  => 0x9a, // UN       => setp  (P=1)
                    OrderedNotEqual            => 0x95, // LT|GT    => setne (Z=0),
                    UnorderedOrEqual           => 0x94, // UN|EQ    => sete  (Z=1)
                    GreaterThan                => 0x97, // GT       => seta  (C=0&Z=0)
                    GreaterThanOrEqual         => 0x93, // GT|EQ    => setae (C=0)
                    UnorderedOrLessThan        => 0x92, // UN|LT    => setb  (C=1)
                    UnorderedOrLessThanOrEqual => 0x96, // UN|LT|EQ => setbe (Z=1|C=1)
                    Equal |                       // EQ
                    NotEqual |                    // UN|LT|GT
                    LessThan |                    // LT
                    LessThanOrEqual |             // LT|EQ
                    UnorderedOrGreaterThan |      // UN|GT
                    UnorderedOrGreaterThanOrEqual // UN|GT|EQ
                    => panic!("{} not supported by fcscc", cond),
                };
                sink.put1(0x0f);
                sink.put1(setcc);
                modrm_rr(out_reg0, 0, sink);
                return;
            }
        }
        // Recipe Op2fcmp
        255 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_op2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                return;
            }
        }
        // Recipe RexOp2fcmp
        256 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_rexop2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                return;
            }
        }
        // Recipe Mp2fcmp
        257 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_mp2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                return;
            }
        }
        // Recipe RexMp2fcmp
        258 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_rexmp2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                return;
            }
        }
        // Recipe Mp3fa
        259 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_mp3(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                return;
            }
        }
        // Recipe Mp2r_ib_unsigned_fpr
        260 => {
            if let InstructionData::ExtractLane {
                opcode,
                lane,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_mp2(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                let imm:i64 = lane.into();
                sink.put1(imm as u8);
                return;
            }
        }
        // Recipe null_fpr
        261 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                return;
            }
        }
        // Recipe Mp3r_ib_unsigned_r
        262 => {
            if let InstructionData::InsertLane {
                opcode,
                lane,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_mp3(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                let imm:i64 = lane.into();
                sink.put1(imm as u8);
                return;
            }
        }
        // Recipe Mp2r_ib_unsigned_r
        263 => {
            if let InstructionData::InsertLane {
                opcode,
                lane,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_mp2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                let imm:i64 = lane.into();
                sink.put1(imm as u8);
                return;
            }
        }
        // Recipe RexMp3r_ib_unsigned_r
        264 => {
            if let InstructionData::InsertLane {
                opcode,
                lane,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_rexmp3(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                let imm:i64 = lane.into();
                sink.put1(imm as u8);
                return;
            }
        }
        // Recipe Mp3fa_ib
        265 => {
            if let InstructionData::InsertLane {
                opcode,
                lane,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_mp3(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                let imm:i64 = lane.into();
                sink.put1(imm as u8);
                return;
            }
        }
        // Recipe Mp3r_ib_unsigned_gpr
        266 => {
            if let InstructionData::ExtractLane {
                opcode,
                lane,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_mp3(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(out_reg0, in_reg0, sink); // note the flipped register in the ModR/M byte
                let imm:i64 = lane.into();
                sink.put1(imm as u8);
                return;
            }
        }
        // Recipe RexMp3r_ib_unsigned_gpr
        267 => {
            if let InstructionData::ExtractLane {
                opcode,
                lane,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexmp3(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(out_reg0, in_reg0, sink); // note the flipped register in the ModR/M byte
                let imm:i64 = lane.into();
                sink.put1(imm as u8);
                return;
            }
        }
        // Recipe Mp2vconst_optimized
        268 => {
            if let InstructionData::UnaryConst {
                opcode,
                constant_handle,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_mp2(bits, rex2(out_reg0, out_reg0), sink);
                modrm_rr(out_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe Op2vconst
        269 => {
            if let InstructionData::UnaryConst {
                opcode,
                constant_handle,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_op2(bits, rex2(0, out_reg0), sink);
                modrm_riprel(out_reg0, sink);
                const_disp4(constant_handle, func, sink);
                return;
            }
        }
        // Recipe Op2fst
        270 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op2(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else if needs_offset(in_reg1) {
                    modrm_disp8(in_reg1, in_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_rm(in_reg1, in_reg0, sink);
                }
                return;
            }
        }
        // Recipe Op2fstDisp8
        271 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op2(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib_disp8(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else {
                    modrm_disp8(in_reg1, in_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe Op2fstDisp32
        272 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op2(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib_disp32(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else {
                    modrm_disp32(in_reg1, in_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe Op2fld
        273 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op2(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else if needs_offset(in_reg0) {
                    modrm_disp8(in_reg0, out_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_rm(in_reg0, out_reg0, sink);
                }
                return;
            }
        }
        // Recipe Op2fldDisp8
        274 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op2(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib_disp8(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else {
                    modrm_disp8(in_reg0, out_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe Op2fldDisp32
        275 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op2(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib_disp32(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else {
                    modrm_disp32(in_reg0, out_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe Op2fspillSib32
        276 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_stk0 = StackRef::masked(
                    divert.stack(results[0], &func.locations),
                    StackBaseMask(1),
                    &func.stack_slots,
                ).unwrap();
                sink.trap(TrapCode::StackOverflow, func.srclocs[inst]);
                let base = stk_base(out_stk0.base);
                put_op2(bits, rex2(base, in_reg0), sink);
                modrm_sib_disp32(in_reg0, sink);
                sib_noindex(base, sink);
                sink.put4(out_stk0.offset as u32);
                return;
            }
        }
        // Recipe Op2fregspill32
        277 => {
            if let InstructionData::RegSpill {
                opcode,
                src,
                dst,
                arg,
                ..
            } = *inst_data {
                divert.apply(inst_data);
                sink.trap(TrapCode::StackOverflow, func.srclocs[inst]);
                let dst = StackRef::sp(dst, &func.stack_slots);
                let base = stk_base(dst.base);
                put_op2(bits, rex2(base, src), sink);
                modrm_sib_disp32(src, sink);
                sib_noindex(base, sink);
                sink.put4(dst.offset as u32);
                return;
            }
        }
        // Recipe Op2ffillSib32
        278 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_stk0 = StackRef::masked(
                    divert.stack(args[0], &func.locations),
                    StackBaseMask(1),
                    &func.stack_slots,
                ).unwrap();
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                let base = stk_base(in_stk0.base);
                put_op2(bits, rex2(base, out_reg0), sink);
                modrm_sib_disp32(out_reg0, sink);
                sib_noindex(base, sink);
                sink.put4(in_stk0.offset as u32);
                return;
            }
        }
        // Recipe Op2fregfill32
        279 => {
            if let InstructionData::RegFill {
                opcode,
                src,
                dst,
                arg,
                ..
            } = *inst_data {
                divert.apply(inst_data);
                let src = StackRef::sp(src, &func.stack_slots);
                let base = stk_base(src.base);
                put_op2(bits, rex2(base, dst), sink);
                modrm_sib_disp32(dst, sink);
                sib_noindex(base, sink);
                sink.put4(src.offset as u32);
                return;
            }
        }
        // Recipe Mp2fax
        280 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_mp2(bits, rex2(in_reg0, in_reg1), sink);
                modrm_rr(in_reg0, in_reg1, sink);
                return;
            }
        }
        // Recipe Mp3fcmp
        281 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_mp3(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                return;
            }
        }
        // Recipe Mp2f_ib
        282 => {
            if let InstructionData::BinaryImm {
                opcode,
                imm,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                put_mp2(bits, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                let imm: i64 = imm.into();
                sink.put1(imm as u8);
                return;
            }
        }
        // Recipe Mp2icscc_fpr
        283 => {
            if let InstructionData::IntCompare {
                opcode,
                cond,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                // Comparison instruction.
                put_mp2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                return;
            }
        }
        // Recipe Mp3icscc_fpr
        284 => {
            if let InstructionData::IntCompare {
                opcode,
                cond,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                // Comparison instruction.
                put_mp3(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                return;
            }
        }
        // Recipe Op2pfcmp
        285 => {
            if let InstructionData::FloatCompare {
                opcode,
                cond,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                // Comparison instruction.
                put_op2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                // Add immediate byte indicating what type of comparison.
                use crate::ir::condcodes::FloatCC::*;
                let imm = match cond {
                    Equal               => 0x00,
                    LessThan            => 0x01,
                    LessThanOrEqual     => 0x02,
                    Unordered           => 0x03,
                    NotEqual            => 0x04,
                    GreaterThanOrEqual  => 0x05,
                    GreaterThan         => 0x06,
                    Ordered             => 0x07,
                    _ => panic!("{} not supported by pfcmp", cond),
                };
                sink.put1(imm);
                return;
            }
        }
        // Recipe RexOp2pfcmp
        286 => {
            if let InstructionData::FloatCompare {
                opcode,
                cond,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                // Comparison instruction.
                put_rexop2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                // Add immediate byte indicating what type of comparison.
                use crate::ir::condcodes::FloatCC::*;
                let imm = match cond {
                    Equal               => 0x00,
                    LessThan            => 0x01,
                    LessThanOrEqual     => 0x02,
                    Unordered           => 0x03,
                    NotEqual            => 0x04,
                    GreaterThanOrEqual  => 0x05,
                    GreaterThan         => 0x06,
                    Ordered             => 0x07,
                    _ => panic!("{} not supported by pfcmp", cond),
                };
                sink.put1(imm);
                return;
            }
        }
        // Recipe Mp2pfcmp
        287 => {
            if let InstructionData::FloatCompare {
                opcode,
                cond,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                // Comparison instruction.
                put_mp2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                // Add immediate byte indicating what type of comparison.
                use crate::ir::condcodes::FloatCC::*;
                let imm = match cond {
                    Equal               => 0x00,
                    LessThan            => 0x01,
                    LessThanOrEqual     => 0x02,
                    Unordered           => 0x03,
                    NotEqual            => 0x04,
                    GreaterThanOrEqual  => 0x05,
                    GreaterThan         => 0x06,
                    Ordered             => 0x07,
                    _ => panic!("{} not supported by pfcmp", cond),
                };
                sink.put1(imm);
                return;
            }
        }
        // Recipe RexMp2pfcmp
        288 => {
            if let InstructionData::FloatCompare {
                opcode,
                cond,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                // Comparison instruction.
                put_rexmp2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                // Add immediate byte indicating what type of comparison.
                use crate::ir::condcodes::FloatCC::*;
                let imm = match cond {
                    Equal               => 0x00,
                    LessThan            => 0x01,
                    LessThanOrEqual     => 0x02,
                    Unordered           => 0x03,
                    NotEqual            => 0x04,
                    GreaterThanOrEqual  => 0x05,
                    GreaterThan         => 0x06,
                    Ordered             => 0x07,
                    _ => panic!("{} not supported by pfcmp", cond),
                };
                sink.put1(imm);
                return;
            }
        }
        // Recipe Op1pu_id_ref
        289 => {
            if let InstructionData::NullAry {
                opcode,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                // The destination register is encoded in the low bits of the opcode.
                // No ModR/M.
                put_op1(bits | (out_reg0 & 7), rex1(out_reg0), sink);
                sink.put4(0);
                return;
            }
        }
        // Recipe RexOp1pu_id_ref
        290 => {
            if let InstructionData::NullAry {
                opcode,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                // The destination register is encoded in the low bits of the opcode.
                // No ModR/M.
                put_rexop1(bits | (out_reg0 & 7), rex1(out_reg0), sink);
                sink.put4(0);
                return;
            }
        }
        // Recipe Op1is_zero
        291 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                // Test instruction.
                put_op1(bits, rex2(in_reg0, in_reg0), sink);
                modrm_rr(in_reg0, in_reg0, sink);
                // Check ZF = 1 flag to see if register holds 0.
                sink.put1(0x0f);
                sink.put1(0x94);
                modrm_rr(out_reg0, 0, sink);
                return;
            }
        }
        // Recipe RexOp1is_zero
        292 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                // Test instruction.
                put_rexop1(bits, rex2(in_reg0, in_reg0), sink);
                modrm_rr(in_reg0, in_reg0, sink);
                // Check ZF = 1 flag to see if register holds 0.
                sink.put1(0x0f);
                sink.put1(0x94);
                modrm_rr(out_reg0, 0, sink);
                return;
            }
        }
        // Recipe safepoint
        293 => {
            if let InstructionData::MultiAry {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                sink.add_stackmap(args, func, isa);
                return;
            }
        }
        _ => {},
    }
    if encoding.is_legal() {
        bad_encoding(func, inst);
    }
}
