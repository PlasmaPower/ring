// Copyright 2016 Brian Smith.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHORS DISCLAIM ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

use super::{CommonOps, EC_GROUP, Elem, Limb, LIMB_BITS, Mont, PublicKeyOps,
            PublicScalarOps};


macro_rules! p384_limbs {
    [$limb_b:expr, $limb_a:expr, $limb_9:expr, $limb_8:expr,
     $limb_7:expr, $limb_6:expr, $limb_5:expr, $limb_4:expr,
     $limb_3:expr, $limb_2:expr, $limb_1:expr, $limb_0:expr] => {
        limbs![$limb_b, $limb_a, $limb_9, $limb_8,
               $limb_7, $limb_6, $limb_5, $limb_4,
               $limb_3, $limb_2, $limb_1, $limb_0]
    };
}


pub static COMMON_OPS: CommonOps = CommonOps {
    num_limbs: 384 / LIMB_BITS,

    q: Mont {
        p: p384_limbs![0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
                       0xffffffff, 0xffffffff, 0xffffffff, 0xfffffffe,
                       0xffffffff, 0x00000000, 0x00000000, 0xffffffff],
        rr: limbs![0, 0, 0, 1, 2, 0, 0xfffffffe, 0, 2, 0, 0xfffffffe, 1 ],
    },

    elem_mul_mont: GFp_p384_elem_mul_mont,
    elem_sqr_mont: GFp_p384_elem_sqr_mont,

    ec_group: &EC_GROUP_P384,
};


pub static PUBLIC_KEY_OPS: PublicKeyOps = PublicKeyOps {
    common: &COMMON_OPS,

    a: Elem {
        limbs: p384_limbs![0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
                           0xffffffff, 0xffffffff, 0xffffffff, 0xfffffffb,
                           0xfffffffc, 0x00000000, 0x00000003, 0xfffffffc],
    },
    b: Elem {
        limbs: p384_limbs![0xcd08114b, 0x604fbff9, 0xb62b21f4, 0x1f022094,
                           0xe3374bee, 0x94938ae2, 0x77f2209b, 0x1920022e,
                           0xf729add8, 0x7a4c32ec, 0x08118871, 0x9d412dcc],
    },

    elem_add_impl: GFp_p384_elem_add,
};


pub static PUBLIC_SCALAR_OPS: PublicScalarOps = PublicScalarOps {
    public_key_ops: &PUBLIC_KEY_OPS,

    n: p384_limbs![0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
                   0xffffffff, 0xffffffff, 0xc7634d81, 0xf4372ddf,
                   0x581a0db2, 0x48b0a77a, 0xecec196a, 0xccc52973],
};


#[allow(non_snake_case)]
unsafe extern fn GFp_p384_elem_sqr_mont(
        r: *mut Limb/*[COMMON_OPS.num_limbs]*/,
        a: *const Limb/*[COMMON_OPS.num_limbs]*/) {
  // XXX: Inefficient. TODO: Make a dedicated squaring routine.
  GFp_p384_elem_mul_mont(r, a, a);
}


extern {
    fn GFp_p384_elem_add(r: *mut Limb/*[COMMON_OPS.num_limbs]*/,
                         a: *const Limb/*[COMMON_OPS.num_limbs]*/,
                         b: *const Limb/*[COMMON_OPS.num_limbs]*/);
    fn GFp_p384_elem_mul_mont(r: *mut Limb/*[COMMON_OPS.num_limbs]*/,
                              a: *const Limb/*[COMMON_OPS.num_limbs]*/,
                              b: *const Limb/*[COMMON_OPS.num_limbs]*/);

    static EC_GROUP_P384: EC_GROUP;
}