/// Legalize instructions by expansion.
///
/// Use x86-specific instructions if needed.
#[allow(unused_variables,unused_assignments,non_snake_case)]
pub fn x86_expand(
    inst: crate::ir::Inst,
    func: &mut crate::ir::Function,
    cfg: &mut crate::flowgraph::ControlFlowGraph,
    isa: &dyn crate::isa::TargetIsa,
) -> bool {
    use crate::ir::InstBuilder;
    use crate::cursor::{Cursor, FuncCursor};
    let mut pos = FuncCursor::new(func).at_inst(inst);
    pos.use_srcloc(inst);
    {
        match pos.func.dfg[inst].opcode() {
            ir::Opcode::Clz => {
                // Unwrap fields from instruction format a := clz.i64(x)
                let (x, args) = if let ir::InstructionData::Unary {
                    arg,
                    ..
                } = pos.func.dfg[inst] {
                    let args = [arg];
                    (
                        pos.func.dfg.resolve_aliases(args[0]),
                        args
                    )
                } else {
                    unreachable!("bad instruction format")
                };

                // Results handled by a := isub(c_sixty_three, index2).
                let r = pos.func.dfg.inst_results(inst);
                let a = &r[0];
                let typeof_a = pos.func.dfg.value_type(*a);

                if pos.func.dfg.value_type(args[0]) == ir::types::I64 {
                    let c_minus_one = pos.ins().iconst(ir::types::I64, -1);
                    let c_sixty_three = pos.ins().iconst(ir::types::I64, 63);
                    let (index1, r2flags) = pos.ins().x86_bsr(x);
                    let index2 = pos.ins().selectif(ir::types::I64, ir::condcodes::IntCC::Equal, r2flags, c_minus_one, index1);
                    let a = pos.func.dfg.replace(inst).isub(c_sixty_three, index2);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::I32 {
                    let c_minus_one = pos.ins().iconst(ir::types::I32, -1);
                    let c_thirty_one = pos.ins().iconst(ir::types::I32, 31);
                    let (index1, r2flags) = pos.ins().x86_bsr(x);
                    let index2 = pos.ins().selectif(ir::types::I32, ir::condcodes::IntCC::Equal, r2flags, c_minus_one, index1);
                    let a = pos.func.dfg.replace(inst).isub(c_thirty_one, index2);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }
            }

            ir::Opcode::Ctz => {
                // Unwrap fields from instruction format a := ctz.i64(x)
                let (x, args) = if let ir::InstructionData::Unary {
                    arg,
                    ..
                } = pos.func.dfg[inst] {
                    let args = [arg];
                    (
                        pos.func.dfg.resolve_aliases(args[0]),
                        args
                    )
                } else {
                    unreachable!("bad instruction format")
                };

                // Results handled by a := selectif(ir::condcodes::IntCC::Equal, r2flags, c_sixty_four, index1).
                let r = pos.func.dfg.inst_results(inst);
                let a = &r[0];
                let typeof_a = pos.func.dfg.value_type(*a);

                if pos.func.dfg.value_type(args[0]) == ir::types::I64 {
                    let c_sixty_four = pos.ins().iconst(ir::types::I64, 64);
                    let (index1, r2flags) = pos.ins().x86_bsf(x);
                    let a = pos.func.dfg.replace(inst).selectif(ir::types::I64, ir::condcodes::IntCC::Equal, r2flags, c_sixty_four, index1);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::I32 {
                    let c_thirty_two = pos.ins().iconst(ir::types::I32, 32);
                    let (index1, r2flags) = pos.ins().x86_bsf(x);
                    let a = pos.func.dfg.replace(inst).selectif(ir::types::I32, ir::condcodes::IntCC::Equal, r2flags, c_thirty_two, index1);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }
            }

            ir::Opcode::Fcmp => {
                // Unwrap fields from instruction format a := fcmp(ir::condcodes::FloatCC::Equal, x, y)
                let (cond, x, y, args) = if let ir::InstructionData::FloatCompare {
                    cond,
                    ref args,
                    ..
                } = pos.func.dfg[inst] {
                    (
                        cond,
                        pos.func.dfg.resolve_aliases(args[0]),
                        pos.func.dfg.resolve_aliases(args[1]),
                        args
                    )
                } else {
                    unreachable!("bad instruction format")
                };

                let typeof_x = pos.func.dfg.value_type(x);
                // Results handled by a := band(a1, a2).
                let r = pos.func.dfg.inst_results(inst);
                let a = &r[0];
                let typeof_a = pos.func.dfg.value_type(*a);

                if predicates::is_equal(cond, ir::condcodes::FloatCC::Equal) {
                    let a1 = pos.ins().fcmp(ir::condcodes::FloatCC::Ordered, x, y);
                    let a2 = pos.ins().fcmp(ir::condcodes::FloatCC::UnorderedOrEqual, x, y);
                    let a = pos.func.dfg.replace(inst).band(a1, a2);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if predicates::is_equal(cond, ir::condcodes::FloatCC::NotEqual) {
                    let a1 = pos.ins().fcmp(ir::condcodes::FloatCC::Unordered, x, y);
                    let a2 = pos.ins().fcmp(ir::condcodes::FloatCC::OrderedNotEqual, x, y);
                    let a = pos.func.dfg.replace(inst).bor(a1, a2);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if predicates::is_equal(cond, ir::condcodes::FloatCC::LessThan) {
                    let a = pos.func.dfg.replace(inst).fcmp(ir::condcodes::FloatCC::GreaterThan, y, x);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if predicates::is_equal(cond, ir::condcodes::FloatCC::LessThanOrEqual) {
                    let a = pos.func.dfg.replace(inst).fcmp(ir::condcodes::FloatCC::GreaterThanOrEqual, y, x);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if predicates::is_equal(cond, ir::condcodes::FloatCC::UnorderedOrGreaterThan) {
                    let a = pos.func.dfg.replace(inst).fcmp(ir::condcodes::FloatCC::UnorderedOrLessThan, y, x);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if predicates::is_equal(cond, ir::condcodes::FloatCC::UnorderedOrGreaterThanOrEqual) {
                    let a = pos.func.dfg.replace(inst).fcmp(ir::condcodes::FloatCC::UnorderedOrLessThanOrEqual, y, x);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }
            }

            ir::Opcode::Popcnt => {
                // Unwrap fields from instruction format r := popcnt.i64(x)
                let (x, args) = if let ir::InstructionData::Unary {
                    arg,
                    ..
                } = pos.func.dfg[inst] {
                    let args = [arg];
                    (
                        pos.func.dfg.resolve_aliases(args[0]),
                        args
                    )
                } else {
                    unreachable!("bad instruction format")
                };

                // Results handled by r := ushr_imm(qv15, 56).
                let r = pos.func.dfg.inst_results(inst);
                let r = &r[0];
                let typeof_r = pos.func.dfg.value_type(*r);

                if pos.func.dfg.value_type(args[0]) == ir::types::I64 {
                    let qv3 = pos.ins().ushr_imm(x, 1);
                    let qc77 = pos.ins().iconst(ir::types::I64, 8608480567731124087);
                    let qv4 = pos.ins().band(qv3, qc77);
                    let qv5 = pos.ins().isub(x, qv4);
                    let qv6 = pos.ins().ushr_imm(qv4, 1);
                    let qv7 = pos.ins().band(qv6, qc77);
                    let qv8 = pos.ins().isub(qv5, qv7);
                    let qv9 = pos.ins().ushr_imm(qv7, 1);
                    let qv10 = pos.ins().band(qv9, qc77);
                    let qv11 = pos.ins().isub(qv8, qv10);
                    let qv12 = pos.ins().ushr_imm(qv11, 4);
                    let qv13 = pos.ins().iadd(qv11, qv12);
                    let qc0F = pos.ins().iconst(ir::types::I64, 1085102592571150095);
                    let qv14 = pos.ins().band(qv13, qc0F);
                    let qc01 = pos.ins().iconst(ir::types::I64, 72340172838076673);
                    let qv15 = pos.ins().imul(qv14, qc01);
                    let r = pos.func.dfg.replace(inst).ushr_imm(qv15, 56);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::I32 {
                    let lv3 = pos.ins().ushr_imm(x, 1);
                    let lc77 = pos.ins().iconst(ir::types::I32, 2004318071);
                    let lv4 = pos.ins().band(lv3, lc77);
                    let lv5 = pos.ins().isub(x, lv4);
                    let lv6 = pos.ins().ushr_imm(lv4, 1);
                    let lv7 = pos.ins().band(lv6, lc77);
                    let lv8 = pos.ins().isub(lv5, lv7);
                    let lv9 = pos.ins().ushr_imm(lv7, 1);
                    let lv10 = pos.ins().band(lv9, lc77);
                    let lv11 = pos.ins().isub(lv8, lv10);
                    let lv12 = pos.ins().ushr_imm(lv11, 4);
                    let lv13 = pos.ins().iadd(lv11, lv12);
                    let lc0F = pos.ins().iconst(ir::types::I32, 252645135);
                    let lv14 = pos.ins().band(lv13, lc0F);
                    let lc01 = pos.ins().iconst(ir::types::I32, 16843009);
                    let lv15 = pos.ins().imul(lv14, lc01);
                    let r = pos.func.dfg.replace(inst).ushr_imm(lv15, 24);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }
            }

            ir::Opcode::Smulhi => {
                // Unwrap fields from instruction format res_hi := smulhi(x, y)
                let (x, y, args) = if let ir::InstructionData::Binary {
                    ref args,
                    ..
                } = pos.func.dfg[inst] {
                    (
                        pos.func.dfg.resolve_aliases(args[0]),
                        pos.func.dfg.resolve_aliases(args[1]),
                        args
                    )
                } else {
                    unreachable!("bad instruction format")
                };

                let typeof_x = pos.func.dfg.value_type(x);
                let res_hi;
                {
                    let r = pos.func.dfg.inst_results(inst);
                    res_hi = r[0];
                }

                let predicate = true;
                // typeof_x must belong to TypeSet(lanes={1}, ints={32, 64})
                let predicate = predicate && TYPE_SETS[0].contains(typeof_x);
                if predicate {
                    pos.func.dfg.clear_results(inst);
                    let (res_lo, res_hi) = pos.ins().with_results([None, Some(res_hi)]).x86_smulx(x, y);
                    let removed = pos.remove_inst();
                    debug_assert_eq!(removed, inst);
                    return true;
                }
            }

            ir::Opcode::Umulhi => {
                // Unwrap fields from instruction format res_hi := umulhi(x, y)
                let (x, y, args) = if let ir::InstructionData::Binary {
                    ref args,
                    ..
                } = pos.func.dfg[inst] {
                    (
                        pos.func.dfg.resolve_aliases(args[0]),
                        pos.func.dfg.resolve_aliases(args[1]),
                        args
                    )
                } else {
                    unreachable!("bad instruction format")
                };

                let typeof_x = pos.func.dfg.value_type(x);
                let res_hi;
                {
                    let r = pos.func.dfg.inst_results(inst);
                    res_hi = r[0];
                }

                let predicate = true;
                // typeof_x must belong to TypeSet(lanes={1}, ints={32, 64})
                let predicate = predicate && TYPE_SETS[0].contains(typeof_x);
                if predicate {
                    pos.func.dfg.clear_results(inst);
                    let (res_lo, res_hi) = pos.ins().with_results([None, Some(res_hi)]).x86_umulx(x, y);
                    let removed = pos.remove_inst();
                    debug_assert_eq!(removed, inst);
                    return true;
                }
            }

            ir::Opcode::FcvtFromUint => {
                expand_fcvt_from_uint(inst, func, cfg, isa);
                return true;
            }

            ir::Opcode::FcvtToSint => {
                expand_fcvt_to_sint(inst, func, cfg, isa);
                return true;
            }

            ir::Opcode::FcvtToSintSat => {
                expand_fcvt_to_sint_sat(inst, func, cfg, isa);
                return true;
            }

            ir::Opcode::FcvtToUint => {
                expand_fcvt_to_uint(inst, func, cfg, isa);
                return true;
            }

            ir::Opcode::FcvtToUintSat => {
                expand_fcvt_to_uint_sat(inst, func, cfg, isa);
                return true;
            }

            ir::Opcode::Fmax => {
                expand_minmax(inst, func, cfg, isa);
                return true;
            }

            ir::Opcode::Fmin => {
                expand_minmax(inst, func, cfg, isa);
                return true;
            }

            ir::Opcode::Sdiv => {
                expand_sdivrem(inst, func, cfg, isa);
                return true;
            }

            ir::Opcode::Srem => {
                expand_sdivrem(inst, func, cfg, isa);
                return true;
            }

            ir::Opcode::Udiv => {
                expand_udivrem(inst, func, cfg, isa);
                return true;
            }

            ir::Opcode::Urem => {
                expand_udivrem(inst, func, cfg, isa);
                return true;
            }

            _ => {},
        }
    }
    crate::legalizer::expand_flags(inst, func, cfg, isa)
}

/// Legalize instructions by narrowing.
///
/// Use x86-specific instructions if needed.
#[allow(unused_variables,unused_assignments,non_snake_case)]
pub fn x86_narrow(
    inst: crate::ir::Inst,
    func: &mut crate::ir::Function,
    cfg: &mut crate::flowgraph::ControlFlowGraph,
    isa: &dyn crate::isa::TargetIsa,
) -> bool {
    use crate::ir::InstBuilder;
    use crate::cursor::{Cursor, FuncCursor};
    let mut pos = FuncCursor::new(func).at_inst(inst);
    pos.use_srcloc(inst);
    {
        match pos.func.dfg[inst].opcode() {
            ir::Opcode::Bitselect => {
                // Unwrap fields from instruction format d := bitselect.b8x16(c, x, y)
                let (c, x, y, args) = if let ir::InstructionData::Ternary {
                    ref args,
                    ..
                } = pos.func.dfg[inst] {
                    (
                        pos.func.dfg.resolve_aliases(args[0]),
                        pos.func.dfg.resolve_aliases(args[1]),
                        pos.func.dfg.resolve_aliases(args[2]),
                        args
                    )
                } else {
                    unreachable!("bad instruction format")
                };

                // Results handled by d := bor(a, b).
                let r = pos.func.dfg.inst_results(inst);
                let d = &r[0];
                let typeof_d = pos.func.dfg.value_type(*d);

                if pos.func.dfg.value_type(args[0]) == ir::types::B8X16 {
                    let a = pos.ins().band(x, c);
                    let b = pos.ins().band_not(y, c);
                    let d = pos.func.dfg.replace(inst).bor(a, b);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::B16X8 {
                    let a = pos.ins().band(x, c);
                    let b = pos.ins().band_not(y, c);
                    let d = pos.func.dfg.replace(inst).bor(a, b);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::B32X4 {
                    let a = pos.ins().band(x, c);
                    let b = pos.ins().band_not(y, c);
                    let d = pos.func.dfg.replace(inst).bor(a, b);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::B64X2 {
                    let a = pos.ins().band(x, c);
                    let b = pos.ins().band_not(y, c);
                    let d = pos.func.dfg.replace(inst).bor(a, b);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::I8X16 {
                    let a = pos.ins().band(x, c);
                    let b = pos.ins().band_not(y, c);
                    let d = pos.func.dfg.replace(inst).bor(a, b);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::I16X8 {
                    let a = pos.ins().band(x, c);
                    let b = pos.ins().band_not(y, c);
                    let d = pos.func.dfg.replace(inst).bor(a, b);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::I32X4 {
                    let a = pos.ins().band(x, c);
                    let b = pos.ins().band_not(y, c);
                    let d = pos.func.dfg.replace(inst).bor(a, b);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::I64X2 {
                    let a = pos.ins().band(x, c);
                    let b = pos.ins().band_not(y, c);
                    let d = pos.func.dfg.replace(inst).bor(a, b);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::F32X4 {
                    let a = pos.ins().band(x, c);
                    let b = pos.ins().band_not(y, c);
                    let d = pos.func.dfg.replace(inst).bor(a, b);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::F64X2 {
                    let a = pos.ins().band(x, c);
                    let b = pos.ins().band_not(y, c);
                    let d = pos.func.dfg.replace(inst).bor(a, b);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }
            }

            ir::Opcode::Bnot => {
                // Unwrap fields from instruction format y := bnot.b8x16(x)
                let (x, args) = if let ir::InstructionData::Unary {
                    arg,
                    ..
                } = pos.func.dfg[inst] {
                    let args = [arg];
                    (
                        pos.func.dfg.resolve_aliases(args[0]),
                        args
                    )
                } else {
                    unreachable!("bad instruction format")
                };

                // Results handled by y := bxor(a, x).
                let r = pos.func.dfg.inst_results(inst);
                let y = &r[0];
                let typeof_y = pos.func.dfg.value_type(*y);

                if pos.func.dfg.value_type(args[0]) == ir::types::B8X16 {
                    let const0 = pos.func.dfg.constants.insert(vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255].into());
                    let a = pos.ins().vconst(ir::types::B8X16, const0);
                    let y = pos.func.dfg.replace(inst).bxor(a, x);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::B16X8 {
                    let const0 = pos.func.dfg.constants.insert(vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255].into());
                    let a = pos.ins().vconst(ir::types::B16X8, const0);
                    let y = pos.func.dfg.replace(inst).bxor(a, x);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::B32X4 {
                    let const0 = pos.func.dfg.constants.insert(vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255].into());
                    let a = pos.ins().vconst(ir::types::B32X4, const0);
                    let y = pos.func.dfg.replace(inst).bxor(a, x);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::B64X2 {
                    let const0 = pos.func.dfg.constants.insert(vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255].into());
                    let a = pos.ins().vconst(ir::types::B64X2, const0);
                    let y = pos.func.dfg.replace(inst).bxor(a, x);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::I8X16 {
                    let const0 = pos.func.dfg.constants.insert(vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255].into());
                    let a = pos.ins().vconst(ir::types::I8X16, const0);
                    let y = pos.func.dfg.replace(inst).bxor(a, x);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::I16X8 {
                    let const0 = pos.func.dfg.constants.insert(vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255].into());
                    let a = pos.ins().vconst(ir::types::I16X8, const0);
                    let y = pos.func.dfg.replace(inst).bxor(a, x);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::I32X4 {
                    let const0 = pos.func.dfg.constants.insert(vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255].into());
                    let a = pos.ins().vconst(ir::types::I32X4, const0);
                    let y = pos.func.dfg.replace(inst).bxor(a, x);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::I64X2 {
                    let const0 = pos.func.dfg.constants.insert(vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255].into());
                    let a = pos.ins().vconst(ir::types::I64X2, const0);
                    let y = pos.func.dfg.replace(inst).bxor(a, x);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::F32X4 {
                    let const0 = pos.func.dfg.constants.insert(vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255].into());
                    let a = pos.ins().vconst(ir::types::F32X4, const0);
                    let y = pos.func.dfg.replace(inst).bxor(a, x);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::F64X2 {
                    let const0 = pos.func.dfg.constants.insert(vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255].into());
                    let a = pos.ins().vconst(ir::types::F64X2, const0);
                    let y = pos.func.dfg.replace(inst).bxor(a, x);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }
            }

            ir::Opcode::Fabs => {
                // Unwrap fields from instruction format b := fabs.f32x4(a)
                let (a, args) = if let ir::InstructionData::Unary {
                    arg,
                    ..
                } = pos.func.dfg[inst] {
                    let args = [arg];
                    (
                        pos.func.dfg.resolve_aliases(args[0]),
                        args
                    )
                } else {
                    unreachable!("bad instruction format")
                };

                // Results handled by b := band(a, e).
                let r = pos.func.dfg.inst_results(inst);
                let b = &r[0];
                let typeof_b = pos.func.dfg.value_type(*b);

                if pos.func.dfg.value_type(args[0]) == ir::types::F32X4 {
                    let const0 = pos.func.dfg.constants.insert(vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255].into());
                    let c = pos.ins().vconst(ir::types::I32X4, const0);
                    let d = pos.ins().ushr_imm(c, 1);
                    let e = pos.ins().raw_bitcast(ir::types::F32X4, d);
                    let b = pos.func.dfg.replace(inst).band(a, e);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::F64X2 {
                    let const0 = pos.func.dfg.constants.insert(vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255].into());
                    let c = pos.ins().vconst(ir::types::I64X2, const0);
                    let d = pos.ins().ushr_imm(c, 1);
                    let e = pos.ins().raw_bitcast(ir::types::F64X2, d);
                    let b = pos.func.dfg.replace(inst).band(a, e);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }
            }

            ir::Opcode::Fneg => {
                // Unwrap fields from instruction format b := fneg.f32x4(a)
                let (a, args) = if let ir::InstructionData::Unary {
                    arg,
                    ..
                } = pos.func.dfg[inst] {
                    let args = [arg];
                    (
                        pos.func.dfg.resolve_aliases(args[0]),
                        args
                    )
                } else {
                    unreachable!("bad instruction format")
                };

                // Results handled by b := bxor(a, e).
                let r = pos.func.dfg.inst_results(inst);
                let b = &r[0];
                let typeof_b = pos.func.dfg.value_type(*b);

                if pos.func.dfg.value_type(args[0]) == ir::types::F32X4 {
                    let const0 = pos.func.dfg.constants.insert(vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255].into());
                    let c = pos.ins().vconst(ir::types::I32X4, const0);
                    let d = pos.ins().ishl_imm(c, 31);
                    let e = pos.ins().raw_bitcast(ir::types::F32X4, d);
                    let b = pos.func.dfg.replace(inst).bxor(a, e);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::F64X2 {
                    let const0 = pos.func.dfg.constants.insert(vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255].into());
                    let c = pos.ins().vconst(ir::types::I64X2, const0);
                    let d = pos.ins().ishl_imm(c, 63);
                    let e = pos.ins().raw_bitcast(ir::types::F64X2, d);
                    let b = pos.func.dfg.replace(inst).bxor(a, e);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }
            }

            ir::Opcode::Icmp => {
                // Unwrap fields from instruction format c := icmp.i8x16(ir::condcodes::IntCC::NotEqual, a, b)
                let (cond, a, b, args) = if let ir::InstructionData::IntCompare {
                    cond,
                    ref args,
                    ..
                } = pos.func.dfg[inst] {
                    (
                        cond,
                        pos.func.dfg.resolve_aliases(args[0]),
                        pos.func.dfg.resolve_aliases(args[1]),
                        args
                    )
                } else {
                    unreachable!("bad instruction format")
                };

                // Results handled by c := bnot(x).
                let r = pos.func.dfg.inst_results(inst);
                let c = &r[0];
                let typeof_c = pos.func.dfg.value_type(*c);

                if predicates::is_equal(cond, ir::condcodes::IntCC::NotEqual) && pos.func.dfg.value_type(args[0]) == ir::types::I8X16 {
                    let x = pos.ins().icmp(ir::condcodes::IntCC::Equal, a, b);
                    let c = pos.func.dfg.replace(inst).bnot(x);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if predicates::is_equal(cond, ir::condcodes::IntCC::NotEqual) && pos.func.dfg.value_type(args[0]) == ir::types::I16X8 {
                    let x = pos.ins().icmp(ir::condcodes::IntCC::Equal, a, b);
                    let c = pos.func.dfg.replace(inst).bnot(x);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if predicates::is_equal(cond, ir::condcodes::IntCC::NotEqual) && pos.func.dfg.value_type(args[0]) == ir::types::I32X4 {
                    let x = pos.ins().icmp(ir::condcodes::IntCC::Equal, a, b);
                    let c = pos.func.dfg.replace(inst).bnot(x);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if predicates::is_equal(cond, ir::condcodes::IntCC::NotEqual) && pos.func.dfg.value_type(args[0]) == ir::types::I64X2 {
                    let x = pos.ins().icmp(ir::condcodes::IntCC::Equal, a, b);
                    let c = pos.func.dfg.replace(inst).bnot(x);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if predicates::is_equal(cond, ir::condcodes::IntCC::UnsignedGreaterThan) && pos.func.dfg.value_type(args[0]) == ir::types::I8X16 {
                    let x = pos.ins().x86_pmaxu(a, b);
                    let y = pos.ins().icmp(ir::condcodes::IntCC::Equal, x, b);
                    let c = pos.func.dfg.replace(inst).bnot(y);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if predicates::is_equal(cond, ir::condcodes::IntCC::SignedGreaterThanOrEqual) && pos.func.dfg.value_type(args[0]) == ir::types::I8X16 {
                    let x = pos.ins().x86_pmins(a, b);
                    let c = pos.func.dfg.replace(inst).icmp(ir::condcodes::IntCC::Equal, x, b);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if predicates::is_equal(cond, ir::condcodes::IntCC::UnsignedGreaterThanOrEqual) && pos.func.dfg.value_type(args[0]) == ir::types::I8X16 {
                    let x = pos.ins().x86_pminu(a, b);
                    let c = pos.func.dfg.replace(inst).icmp(ir::condcodes::IntCC::Equal, x, b);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if predicates::is_equal(cond, ir::condcodes::IntCC::SignedLessThan) && pos.func.dfg.value_type(args[0]) == ir::types::I8X16 {
                    let c = pos.func.dfg.replace(inst).icmp(ir::condcodes::IntCC::SignedGreaterThan, b, a);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if predicates::is_equal(cond, ir::condcodes::IntCC::UnsignedLessThan) && pos.func.dfg.value_type(args[0]) == ir::types::I8X16 {
                    let c = pos.func.dfg.replace(inst).icmp(ir::condcodes::IntCC::UnsignedGreaterThan, b, a);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if predicates::is_equal(cond, ir::condcodes::IntCC::SignedLessThanOrEqual) && pos.func.dfg.value_type(args[0]) == ir::types::I8X16 {
                    let c = pos.func.dfg.replace(inst).icmp(ir::condcodes::IntCC::SignedGreaterThanOrEqual, b, a);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if predicates::is_equal(cond, ir::condcodes::IntCC::UnsignedLessThanOrEqual) && pos.func.dfg.value_type(args[0]) == ir::types::I8X16 {
                    let c = pos.func.dfg.replace(inst).icmp(ir::condcodes::IntCC::UnsignedGreaterThanOrEqual, b, a);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if predicates::is_equal(cond, ir::condcodes::IntCC::UnsignedGreaterThan) && pos.func.dfg.value_type(args[0]) == ir::types::I16X8 {
                    let x = pos.ins().x86_pmaxu(a, b);
                    let y = pos.ins().icmp(ir::condcodes::IntCC::Equal, x, b);
                    let c = pos.func.dfg.replace(inst).bnot(y);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if predicates::is_equal(cond, ir::condcodes::IntCC::SignedGreaterThanOrEqual) && pos.func.dfg.value_type(args[0]) == ir::types::I16X8 {
                    let x = pos.ins().x86_pmins(a, b);
                    let c = pos.func.dfg.replace(inst).icmp(ir::condcodes::IntCC::Equal, x, b);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if predicates::is_equal(cond, ir::condcodes::IntCC::UnsignedGreaterThanOrEqual) && pos.func.dfg.value_type(args[0]) == ir::types::I16X8 {
                    let x = pos.ins().x86_pminu(a, b);
                    let c = pos.func.dfg.replace(inst).icmp(ir::condcodes::IntCC::Equal, x, b);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if predicates::is_equal(cond, ir::condcodes::IntCC::SignedLessThan) && pos.func.dfg.value_type(args[0]) == ir::types::I16X8 {
                    let c = pos.func.dfg.replace(inst).icmp(ir::condcodes::IntCC::SignedGreaterThan, b, a);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if predicates::is_equal(cond, ir::condcodes::IntCC::UnsignedLessThan) && pos.func.dfg.value_type(args[0]) == ir::types::I16X8 {
                    let c = pos.func.dfg.replace(inst).icmp(ir::condcodes::IntCC::UnsignedGreaterThan, b, a);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if predicates::is_equal(cond, ir::condcodes::IntCC::SignedLessThanOrEqual) && pos.func.dfg.value_type(args[0]) == ir::types::I16X8 {
                    let c = pos.func.dfg.replace(inst).icmp(ir::condcodes::IntCC::SignedGreaterThanOrEqual, b, a);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if predicates::is_equal(cond, ir::condcodes::IntCC::UnsignedLessThanOrEqual) && pos.func.dfg.value_type(args[0]) == ir::types::I16X8 {
                    let c = pos.func.dfg.replace(inst).icmp(ir::condcodes::IntCC::UnsignedGreaterThanOrEqual, b, a);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if predicates::is_equal(cond, ir::condcodes::IntCC::UnsignedGreaterThan) && pos.func.dfg.value_type(args[0]) == ir::types::I32X4 {
                    let x = pos.ins().x86_pmaxu(a, b);
                    let y = pos.ins().icmp(ir::condcodes::IntCC::Equal, x, b);
                    let c = pos.func.dfg.replace(inst).bnot(y);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if predicates::is_equal(cond, ir::condcodes::IntCC::SignedGreaterThanOrEqual) && pos.func.dfg.value_type(args[0]) == ir::types::I32X4 {
                    let x = pos.ins().x86_pmins(a, b);
                    let c = pos.func.dfg.replace(inst).icmp(ir::condcodes::IntCC::Equal, x, b);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if predicates::is_equal(cond, ir::condcodes::IntCC::UnsignedGreaterThanOrEqual) && pos.func.dfg.value_type(args[0]) == ir::types::I32X4 {
                    let x = pos.ins().x86_pminu(a, b);
                    let c = pos.func.dfg.replace(inst).icmp(ir::condcodes::IntCC::Equal, x, b);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if predicates::is_equal(cond, ir::condcodes::IntCC::SignedLessThan) && pos.func.dfg.value_type(args[0]) == ir::types::I32X4 {
                    let c = pos.func.dfg.replace(inst).icmp(ir::condcodes::IntCC::SignedGreaterThan, b, a);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if predicates::is_equal(cond, ir::condcodes::IntCC::UnsignedLessThan) && pos.func.dfg.value_type(args[0]) == ir::types::I32X4 {
                    let c = pos.func.dfg.replace(inst).icmp(ir::condcodes::IntCC::UnsignedGreaterThan, b, a);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if predicates::is_equal(cond, ir::condcodes::IntCC::SignedLessThanOrEqual) && pos.func.dfg.value_type(args[0]) == ir::types::I32X4 {
                    let c = pos.func.dfg.replace(inst).icmp(ir::condcodes::IntCC::SignedGreaterThanOrEqual, b, a);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if predicates::is_equal(cond, ir::condcodes::IntCC::UnsignedLessThanOrEqual) && pos.func.dfg.value_type(args[0]) == ir::types::I32X4 {
                    let c = pos.func.dfg.replace(inst).icmp(ir::condcodes::IntCC::UnsignedGreaterThanOrEqual, b, a);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }
            }

            ir::Opcode::Ishl => {
                // Unwrap fields from instruction format a := ishl.i16x8(x, y)
                let (x, y, args) = if let ir::InstructionData::Binary {
                    ref args,
                    ..
                } = pos.func.dfg[inst] {
                    (
                        pos.func.dfg.resolve_aliases(args[0]),
                        pos.func.dfg.resolve_aliases(args[1]),
                        args
                    )
                } else {
                    unreachable!("bad instruction format")
                };

                let typeof_y = pos.func.dfg.value_type(y);
                // Results handled by a := x86_psll(x, b).
                let r = pos.func.dfg.inst_results(inst);
                let a = &r[0];
                let typeof_a = pos.func.dfg.value_type(*a);

                if pos.func.dfg.value_type(args[0]) == ir::types::I16X8 {
                    let b = pos.ins().bitcast(ir::types::I64X2, y);
                    let a = pos.func.dfg.replace(inst).x86_psll(x, b);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::I32X4 {
                    let b = pos.ins().bitcast(ir::types::I64X2, y);
                    let a = pos.func.dfg.replace(inst).x86_psll(x, b);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::I64X2 {
                    let b = pos.ins().bitcast(ir::types::I64X2, y);
                    let a = pos.func.dfg.replace(inst).x86_psll(x, b);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }
            }

            ir::Opcode::Splat => {
                // Unwrap fields from instruction format y := splat.b8x16(x)
                let (x, args) = if let ir::InstructionData::Unary {
                    arg,
                    ..
                } = pos.func.dfg[inst] {
                    let args = [arg];
                    (
                        pos.func.dfg.resolve_aliases(args[0]),
                        args
                    )
                } else {
                    unreachable!("bad instruction format")
                };

                // Results handled by y := x86_pshufb(a, b).
                let r = pos.func.dfg.inst_results(inst);
                let y = &r[0];
                let typeof_y = pos.func.dfg.value_type(*y);

                if pos.func.dfg.ctrl_typevar(inst) == ir::types::B8X16 {
                    let const0 = pos.func.dfg.constants.insert(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].into());
                    let a = pos.ins().scalar_to_vector(ir::types::B8X16, x);
                    let b = pos.ins().vconst(ir::types::B8X16, const0);
                    let y = pos.func.dfg.replace(inst).x86_pshufb(a, b);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.ctrl_typevar(inst) == ir::types::I8X16 {
                    let const0 = pos.func.dfg.constants.insert(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].into());
                    let a = pos.ins().scalar_to_vector(ir::types::I8X16, x);
                    let b = pos.ins().vconst(ir::types::I8X16, const0);
                    let y = pos.func.dfg.replace(inst).x86_pshufb(a, b);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.ctrl_typevar(inst) == ir::types::B16X8 {
                    let a = pos.ins().scalar_to_vector(ir::types::B16X8, x);
                    let b = pos.ins().insertlane(a, 1, x);
                    let c = pos.ins().raw_bitcast(ir::types::I32X4, b);
                    let d = pos.ins().x86_pshufd(c, 0);
                    let y = pos.func.dfg.replace(inst).raw_bitcast(ir::types::B16X8, d);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.ctrl_typevar(inst) == ir::types::I16X8 {
                    let a = pos.ins().scalar_to_vector(ir::types::I16X8, x);
                    let b = pos.ins().insertlane(a, 1, x);
                    let c = pos.ins().raw_bitcast(ir::types::I32X4, b);
                    let d = pos.ins().x86_pshufd(c, 0);
                    let y = pos.func.dfg.replace(inst).raw_bitcast(ir::types::I16X8, d);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.ctrl_typevar(inst) == ir::types::B32X4 {
                    let a = pos.ins().scalar_to_vector(ir::types::B32X4, x);
                    let y = pos.func.dfg.replace(inst).x86_pshufd(a, 0);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.ctrl_typevar(inst) == ir::types::I32X4 {
                    let a = pos.ins().scalar_to_vector(ir::types::I32X4, x);
                    let y = pos.func.dfg.replace(inst).x86_pshufd(a, 0);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.ctrl_typevar(inst) == ir::types::F32X4 {
                    let a = pos.ins().scalar_to_vector(ir::types::F32X4, x);
                    let y = pos.func.dfg.replace(inst).x86_pshufd(a, 0);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.ctrl_typevar(inst) == ir::types::B64X2 {
                    let a = pos.ins().scalar_to_vector(ir::types::B64X2, x);
                    let y = pos.func.dfg.replace(inst).insertlane(a, 1, x);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.ctrl_typevar(inst) == ir::types::I64X2 {
                    let a = pos.ins().scalar_to_vector(ir::types::I64X2, x);
                    let y = pos.func.dfg.replace(inst).insertlane(a, 1, x);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.ctrl_typevar(inst) == ir::types::F64X2 {
                    let a = pos.ins().scalar_to_vector(ir::types::F64X2, x);
                    let y = pos.func.dfg.replace(inst).insertlane(a, 1, x);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }
            }

            ir::Opcode::Sshr => {
                // Unwrap fields from instruction format a := sshr.i16x8(x, y)
                let (x, y, args) = if let ir::InstructionData::Binary {
                    ref args,
                    ..
                } = pos.func.dfg[inst] {
                    (
                        pos.func.dfg.resolve_aliases(args[0]),
                        pos.func.dfg.resolve_aliases(args[1]),
                        args
                    )
                } else {
                    unreachable!("bad instruction format")
                };

                let typeof_y = pos.func.dfg.value_type(y);
                // Results handled by a := x86_psra(x, b).
                let r = pos.func.dfg.inst_results(inst);
                let a = &r[0];
                let typeof_a = pos.func.dfg.value_type(*a);

                if pos.func.dfg.value_type(args[0]) == ir::types::I16X8 {
                    let b = pos.ins().bitcast(ir::types::I64X2, y);
                    let a = pos.func.dfg.replace(inst).x86_psra(x, b);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::I32X4 {
                    let b = pos.ins().bitcast(ir::types::I64X2, y);
                    let a = pos.func.dfg.replace(inst).x86_psra(x, b);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::I64X2 {
                    let b = pos.ins().bitcast(ir::types::I64X2, y);
                    let a = pos.func.dfg.replace(inst).x86_psra(x, b);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }
            }

            ir::Opcode::Ushr => {
                // Unwrap fields from instruction format a := ushr.i16x8(x, y)
                let (x, y, args) = if let ir::InstructionData::Binary {
                    ref args,
                    ..
                } = pos.func.dfg[inst] {
                    (
                        pos.func.dfg.resolve_aliases(args[0]),
                        pos.func.dfg.resolve_aliases(args[1]),
                        args
                    )
                } else {
                    unreachable!("bad instruction format")
                };

                let typeof_y = pos.func.dfg.value_type(y);
                // Results handled by a := x86_psrl(x, b).
                let r = pos.func.dfg.inst_results(inst);
                let a = &r[0];
                let typeof_a = pos.func.dfg.value_type(*a);

                if pos.func.dfg.value_type(args[0]) == ir::types::I16X8 {
                    let b = pos.ins().bitcast(ir::types::I64X2, y);
                    let a = pos.func.dfg.replace(inst).x86_psrl(x, b);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::I32X4 {
                    let b = pos.ins().bitcast(ir::types::I64X2, y);
                    let a = pos.func.dfg.replace(inst).x86_psrl(x, b);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::I64X2 {
                    let b = pos.ins().bitcast(ir::types::I64X2, y);
                    let a = pos.func.dfg.replace(inst).x86_psrl(x, b);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }
            }

            ir::Opcode::VallTrue => {
                // Unwrap fields from instruction format y := vall_true.b8x16(x)
                let (x, args) = if let ir::InstructionData::Unary {
                    arg,
                    ..
                } = pos.func.dfg[inst] {
                    let args = [arg];
                    (
                        pos.func.dfg.resolve_aliases(args[0]),
                        args
                    )
                } else {
                    unreachable!("bad instruction format")
                };

                // Results handled by y := trueif(ir::condcodes::IntCC::Equal, d).
                let r = pos.func.dfg.inst_results(inst);
                let y = &r[0];
                let typeof_y = pos.func.dfg.value_type(*y);

                if pos.func.dfg.value_type(args[0]) == ir::types::B8X16 {
                    let const0 = pos.func.dfg.constants.insert(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].into());
                    let a = pos.ins().vconst(ir::types::I8X16, const0);
                    let b = pos.ins().raw_bitcast(ir::types::I8X16, x);
                    let c = pos.ins().icmp(ir::condcodes::IntCC::Equal, b, a);
                    let d = pos.ins().x86_ptest(c, c);
                    let y = pos.func.dfg.replace(inst).trueif(ir::condcodes::IntCC::Equal, d);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::B16X8 {
                    let const0 = pos.func.dfg.constants.insert(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].into());
                    let a = pos.ins().vconst(ir::types::I16X8, const0);
                    let b = pos.ins().raw_bitcast(ir::types::I16X8, x);
                    let c = pos.ins().icmp(ir::condcodes::IntCC::Equal, b, a);
                    let d = pos.ins().x86_ptest(c, c);
                    let y = pos.func.dfg.replace(inst).trueif(ir::condcodes::IntCC::Equal, d);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::B32X4 {
                    let const0 = pos.func.dfg.constants.insert(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].into());
                    let a = pos.ins().vconst(ir::types::I32X4, const0);
                    let b = pos.ins().raw_bitcast(ir::types::I32X4, x);
                    let c = pos.ins().icmp(ir::condcodes::IntCC::Equal, b, a);
                    let d = pos.ins().x86_ptest(c, c);
                    let y = pos.func.dfg.replace(inst).trueif(ir::condcodes::IntCC::Equal, d);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::B64X2 {
                    let const0 = pos.func.dfg.constants.insert(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].into());
                    let a = pos.ins().vconst(ir::types::I64X2, const0);
                    let b = pos.ins().raw_bitcast(ir::types::I64X2, x);
                    let c = pos.ins().icmp(ir::condcodes::IntCC::Equal, b, a);
                    let d = pos.ins().x86_ptest(c, c);
                    let y = pos.func.dfg.replace(inst).trueif(ir::condcodes::IntCC::Equal, d);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::I8X16 {
                    let const0 = pos.func.dfg.constants.insert(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].into());
                    let a = pos.ins().vconst(ir::types::I8X16, const0);
                    let c = pos.ins().icmp(ir::condcodes::IntCC::Equal, x, a);
                    let d = pos.ins().x86_ptest(c, c);
                    let y = pos.func.dfg.replace(inst).trueif(ir::condcodes::IntCC::Equal, d);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::I16X8 {
                    let const0 = pos.func.dfg.constants.insert(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].into());
                    let a = pos.ins().vconst(ir::types::I16X8, const0);
                    let c = pos.ins().icmp(ir::condcodes::IntCC::Equal, x, a);
                    let d = pos.ins().x86_ptest(c, c);
                    let y = pos.func.dfg.replace(inst).trueif(ir::condcodes::IntCC::Equal, d);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::I32X4 {
                    let const0 = pos.func.dfg.constants.insert(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].into());
                    let a = pos.ins().vconst(ir::types::I32X4, const0);
                    let c = pos.ins().icmp(ir::condcodes::IntCC::Equal, x, a);
                    let d = pos.ins().x86_ptest(c, c);
                    let y = pos.func.dfg.replace(inst).trueif(ir::condcodes::IntCC::Equal, d);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::I64X2 {
                    let const0 = pos.func.dfg.constants.insert(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].into());
                    let a = pos.ins().vconst(ir::types::I64X2, const0);
                    let c = pos.ins().icmp(ir::condcodes::IntCC::Equal, x, a);
                    let d = pos.ins().x86_ptest(c, c);
                    let y = pos.func.dfg.replace(inst).trueif(ir::condcodes::IntCC::Equal, d);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::F32X4 {
                    let const0 = pos.func.dfg.constants.insert(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].into());
                    let a = pos.ins().vconst(ir::types::I32X4, const0);
                    let b = pos.ins().raw_bitcast(ir::types::I32X4, x);
                    let c = pos.ins().icmp(ir::condcodes::IntCC::Equal, b, a);
                    let d = pos.ins().x86_ptest(c, c);
                    let y = pos.func.dfg.replace(inst).trueif(ir::condcodes::IntCC::Equal, d);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::F64X2 {
                    let const0 = pos.func.dfg.constants.insert(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].into());
                    let a = pos.ins().vconst(ir::types::I64X2, const0);
                    let b = pos.ins().raw_bitcast(ir::types::I64X2, x);
                    let c = pos.ins().icmp(ir::condcodes::IntCC::Equal, b, a);
                    let d = pos.ins().x86_ptest(c, c);
                    let y = pos.func.dfg.replace(inst).trueif(ir::condcodes::IntCC::Equal, d);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }
            }

            ir::Opcode::VanyTrue => {
                // Unwrap fields from instruction format y := vany_true.b8x16(x)
                let (x, args) = if let ir::InstructionData::Unary {
                    arg,
                    ..
                } = pos.func.dfg[inst] {
                    let args = [arg];
                    (
                        pos.func.dfg.resolve_aliases(args[0]),
                        args
                    )
                } else {
                    unreachable!("bad instruction format")
                };

                // Results handled by y := trueif(ir::condcodes::IntCC::NotEqual, a).
                let r = pos.func.dfg.inst_results(inst);
                let y = &r[0];
                let typeof_y = pos.func.dfg.value_type(*y);

                if pos.func.dfg.value_type(args[0]) == ir::types::B8X16 {
                    let a = pos.ins().x86_ptest(x, x);
                    let y = pos.func.dfg.replace(inst).trueif(ir::condcodes::IntCC::NotEqual, a);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::B16X8 {
                    let a = pos.ins().x86_ptest(x, x);
                    let y = pos.func.dfg.replace(inst).trueif(ir::condcodes::IntCC::NotEqual, a);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::B32X4 {
                    let a = pos.ins().x86_ptest(x, x);
                    let y = pos.func.dfg.replace(inst).trueif(ir::condcodes::IntCC::NotEqual, a);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::B64X2 {
                    let a = pos.ins().x86_ptest(x, x);
                    let y = pos.func.dfg.replace(inst).trueif(ir::condcodes::IntCC::NotEqual, a);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::I8X16 {
                    let a = pos.ins().x86_ptest(x, x);
                    let y = pos.func.dfg.replace(inst).trueif(ir::condcodes::IntCC::NotEqual, a);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::I16X8 {
                    let a = pos.ins().x86_ptest(x, x);
                    let y = pos.func.dfg.replace(inst).trueif(ir::condcodes::IntCC::NotEqual, a);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::I32X4 {
                    let a = pos.ins().x86_ptest(x, x);
                    let y = pos.func.dfg.replace(inst).trueif(ir::condcodes::IntCC::NotEqual, a);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::I64X2 {
                    let a = pos.ins().x86_ptest(x, x);
                    let y = pos.func.dfg.replace(inst).trueif(ir::condcodes::IntCC::NotEqual, a);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::F32X4 {
                    let a = pos.ins().x86_ptest(x, x);
                    let y = pos.func.dfg.replace(inst).trueif(ir::condcodes::IntCC::NotEqual, a);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }

                if pos.func.dfg.value_type(args[0]) == ir::types::F64X2 {
                    let a = pos.ins().x86_ptest(x, x);
                    let y = pos.func.dfg.replace(inst).trueif(ir::condcodes::IntCC::NotEqual, a);
                    if pos.current_inst() == Some(inst) {
                        pos.next_inst();
                    }
                    return true;
                }
            }

            ir::Opcode::Extractlane => {
                convert_extractlane(inst, func, cfg, isa);
                return true;
            }

            ir::Opcode::Ineg => {
                convert_ineg(inst, func, cfg, isa);
                return true;
            }

            ir::Opcode::Insertlane => {
                convert_insertlane(inst, func, cfg, isa);
                return true;
            }

            ir::Opcode::Shuffle => {
                convert_shuffle(inst, func, cfg, isa);
                return true;
            }

            _ => {},
        }
    }
    crate::legalizer::narrow_flags(inst, func, cfg, isa)
}

// Table of value type sets.
const TYPE_SETS: [ir::instructions::ValueTypeSet; 1] = [
    ir::instructions::ValueTypeSet {
        // TypeSet(lanes={1}, ints={32, 64})
        lanes: BitSet::<u16>(1),
        ints: BitSet::<u8>(96),
        floats: BitSet::<u8>(0),
        bools: BitSet::<u8>(0),
        refs: BitSet::<u8>(0),
    },
];
pub static LEGALIZE_ACTIONS: [isa::Legalize; 5] = [
    crate::legalizer::expand_flags,
    crate::legalizer::widen,
    x86_expand,
    x86_narrow,
    crate::legalizer::narrow_flags,
];
