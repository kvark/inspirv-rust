
use rustc::mir;
use rustc::ty::{self, Ty, TypeFoldable, TyCtxt};
use inspirv::core::instruction::*;

use {BlockAndBuilder, MirContext};
use lvalue::{LvalueRef, ValueRef};

#[derive(Debug)]
pub enum OperandValue {
    Immediate(ValueRef),
    Null,
}

#[derive(Debug)]
pub struct OperandRef<'tcx> {
    // The value.
    pub val: OperandValue,

    // The type of value being returned.
    pub ty: Ty<'tcx>
}

impl<'bcx, 'tcx> MirContext<'bcx, 'tcx> {
    pub fn trans_operand(&mut self,
                         bcx: &BlockAndBuilder<'bcx, 'tcx>,
                         operand: &mir::Operand<'tcx>)
                         -> Option<OperandRef<'tcx>>
    {
        println!("trans_operand(operand={:#?})", operand);

        match *operand {
            mir::Operand::Consume(ref lvalue) => {
                self.trans_consume(bcx, lvalue)
            }

            mir::Operand::Constant(ref constant) => {
                let const_val = self.trans_constant(bcx, constant);
                let operand = const_val.to_operand(bcx.ccx());
                Some(operand)
            }
        }
    }

    pub fn trans_load(&mut self,
                      bcx: &BlockAndBuilder<'bcx, 'tcx>,
                      spv_val: ValueRef,
                      ty: Ty<'tcx>)
                      -> OperandRef<'tcx>
    {
        println!("trans_load: {:#?} @ {:#?}", spv_val, ty);
        let mut builder = self.fcx.spv().borrow_mut();
        let operand_id = builder.alloc_id();
        bcx.with_block(|bcx| {
            bcx.spv_block.borrow_mut().emit_instruction(OpLoad(builder.define_type(&spv_val.spvty), operand_id, spv_val.spvid, None))
        });

        OperandRef {
            val: OperandValue::Immediate(ValueRef {
                spvid: operand_id,
                spvty: spv_val.spvty,
            }),
            ty: ty,
        }
    }

    pub fn trans_consume(&mut self,
                         bcx: &BlockAndBuilder<'bcx, 'tcx>,
                         lvalue: &mir::Lvalue<'tcx>)
                         -> Option<OperandRef<'tcx>>
    {
        println!("trans_consume(lvalue={:#?})", lvalue);

        let tr_lvalue = self.trans_lvalue(bcx, lvalue);
        match tr_lvalue {
            LvalueRef::Value(val, ty) => {
                let ty = ty.to_ty(bcx.tcx());
                Some(self.trans_load(bcx, val, ty))
            }
            LvalueRef::Ref { .. } => {
                // unimplemented!(),
                None
            }
            LvalueRef::SigStruct(_, _) => {
                // unimplemented!(),
                None
            }
            LvalueRef::Ignore => None,
        }
    }

    pub fn store_operand(&mut self,
                         bcx: &BlockAndBuilder<'bcx, 'tcx>,
                         dest: LvalueRef,
                         operand: OperandRef<'tcx>)
    {
        println!("store_operand: operand={:#?}", operand);
        bcx.with_block(|bcx| {
            match operand.val {
                OperandValue::Immediate(ref op) => {
                    match dest {
                        LvalueRef::Value(ref lval, _) => {
                            bcx.spv_block.borrow_mut().emit_instruction(
                                OpStore(lval.spvid, op.spvid, None))
                        }
                        _ => unimplemented!(),
                    }  
                }
                OperandValue::Null => {
                    bug!()
                }
            }
        });
    }
}
