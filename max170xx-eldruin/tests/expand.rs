#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod base {
    use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTrans};
    use max170xx::{Max17043, Max17044, Max17048, Max17049, Max17058, Max17059};
    pub const ADDR: u8 = 0b011_0110;
    pub struct Register;
    impl Register {
        pub const VCELL: u8 = 0x02;
        pub const SOC: u8 = 0x04;
        pub const MODE: u8 = 0x06;
        pub const VERSION: u8 = 0x08;
        pub const CRATE: u8 = 0x16;
        pub const COMMAND: u8 = 0xFE;
    }
    pub struct Command;
    impl Command {
        pub const POR_43_44: u16 = 0x0054;
        pub const POR_X8_X9: u16 = 0x5400;
        pub const QSTRT: u16 = 0x4000;
    }
    #[allow(unused)]
    pub fn new_43(transactions: &[I2cTrans]) -> Max17043<I2cMock> {
        Max17043::new(I2cMock::new(transactions))
    }
    #[allow(unused)]
    pub fn destroy_43(sensor: Max17043<I2cMock>) {
        sensor.destroy().done();
    }
    #[allow(unused)]
    pub fn new_44(transactions: &[I2cTrans]) -> Max17044<I2cMock> {
        Max17044::new(I2cMock::new(transactions))
    }
    #[allow(unused)]
    pub fn destroy_44(sensor: Max17044<I2cMock>) {
        sensor.destroy().done();
    }
    #[allow(unused)]
    pub fn new_48(transactions: &[I2cTrans]) -> Max17048<I2cMock> {
        Max17048::new(I2cMock::new(transactions))
    }
    #[allow(unused)]
    pub fn destroy_48(sensor: Max17048<I2cMock>) {
        sensor.destroy().done();
    }
    #[allow(unused)]
    pub fn new_49(transactions: &[I2cTrans]) -> Max17049<I2cMock> {
        Max17049::new(I2cMock::new(transactions))
    }
    #[allow(unused)]
    pub fn destroy_49(sensor: Max17049<I2cMock>) {
        sensor.destroy().done();
    }
    #[allow(unused)]
    pub fn new_58(transactions: &[I2cTrans]) -> Max17058<I2cMock> {
        Max17058::new(I2cMock::new(transactions))
    }
    #[allow(unused)]
    pub fn destroy_58(sensor: Max17058<I2cMock>) {
        sensor.destroy().done();
    }
    #[allow(unused)]
    pub fn new_59(transactions: &[I2cTrans]) -> Max17059<I2cMock> {
        Max17059::new(I2cMock::new(transactions))
    }
    #[allow(unused)]
    pub fn destroy_59(sensor: Max17059<I2cMock>) {
        sensor.destroy().done();
    }
}
use crate::base::{
    destroy_43, destroy_44, destroy_48, destroy_49, destroy_58, destroy_59, new_43,
    new_44, new_48, new_49, new_58, new_59, Command, Register, ADDR,
};
use embedded_hal_mock::eh1::i2c::Transaction as I2cTrans;
mod common {
    use super::*;
    mod max17043 {
        use super::*;
        extern crate test;
        #[cfg(test)]
        #[rustc_test_marker = "common::max17043::can_create_and_destroy"]
        #[doc(hidden)]
        pub const can_create_and_destroy: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("common::max17043::can_create_and_destroy"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "tests/integration.rs",
                start_line: 49usize,
                start_col: 16usize,
                end_line: 49usize,
                end_col: 38usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::IntegrationTest,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(can_create_and_destroy()),
            ),
        };
        fn can_create_and_destroy() {
            let sensor = new_43(&[]);
            destroy_43(sensor);
        }
        extern crate test;
        #[cfg(test)]
        #[rustc_test_marker = "common::max17043::can_get_version"]
        #[doc(hidden)]
        pub const can_get_version: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("common::max17043::can_get_version"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "tests/integration.rs",
                start_line: 55usize,
                start_col: 16usize,
                end_line: 55usize,
                end_col: 31usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::IntegrationTest,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(can_get_version()),
            ),
        };
        fn can_get_version() {
            let version = 0xABCD;
            let mut sensor = new_43(
                &[
                    I2cTrans::write_read(
                        ADDR,
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([Register::VERSION]),
                        ),
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([0xAB, 0xCD]),
                        ),
                    ),
                ],
            );
            let v = sensor.version().unwrap();
            match (&v, &version) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            destroy_43(sensor);
        }
        extern crate test;
        #[cfg(test)]
        #[rustc_test_marker = "common::max17043::quickstart"]
        #[doc(hidden)]
        pub const quickstart: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("common::max17043::quickstart"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "tests/integration.rs",
                start_line: 67usize,
                start_col: 23usize,
                end_line: 67usize,
                end_col: 33usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::IntegrationTest,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(quickstart()),
            ),
        };
        fn quickstart() {
            let mut sensor = new_43(
                &[
                    I2cTrans::write(
                        ADDR,
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                Register::MODE,
                                ((Command::QSTRT & 0xFF00) >> 8) as u8,
                                (Command::QSTRT & 0xFF) as u8,
                            ]),
                        ),
                    ),
                ],
            );
            sensor.quickstart().unwrap();
            destroy_43(sensor);
        }
    }
    mod max17044 {
        use super::*;
        extern crate test;
        #[cfg(test)]
        #[rustc_test_marker = "common::max17044::can_create_and_destroy"]
        #[doc(hidden)]
        pub const can_create_and_destroy: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("common::max17044::can_create_and_destroy"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "tests/integration.rs",
                start_line: 49usize,
                start_col: 16usize,
                end_line: 49usize,
                end_col: 38usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::IntegrationTest,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(can_create_and_destroy()),
            ),
        };
        fn can_create_and_destroy() {
            let sensor = new_44(&[]);
            destroy_44(sensor);
        }
        extern crate test;
        #[cfg(test)]
        #[rustc_test_marker = "common::max17044::can_get_version"]
        #[doc(hidden)]
        pub const can_get_version: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("common::max17044::can_get_version"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "tests/integration.rs",
                start_line: 55usize,
                start_col: 16usize,
                end_line: 55usize,
                end_col: 31usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::IntegrationTest,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(can_get_version()),
            ),
        };
        fn can_get_version() {
            let version = 0xABCD;
            let mut sensor = new_44(
                &[
                    I2cTrans::write_read(
                        ADDR,
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([Register::VERSION]),
                        ),
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([0xAB, 0xCD]),
                        ),
                    ),
                ],
            );
            let v = sensor.version().unwrap();
            match (&v, &version) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            destroy_44(sensor);
        }
        extern crate test;
        #[cfg(test)]
        #[rustc_test_marker = "common::max17044::quickstart"]
        #[doc(hidden)]
        pub const quickstart: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("common::max17044::quickstart"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "tests/integration.rs",
                start_line: 67usize,
                start_col: 23usize,
                end_line: 67usize,
                end_col: 33usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::IntegrationTest,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(quickstart()),
            ),
        };
        fn quickstart() {
            let mut sensor = new_44(
                &[
                    I2cTrans::write(
                        ADDR,
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                Register::MODE,
                                ((Command::QSTRT & 0xFF00) >> 8) as u8,
                                (Command::QSTRT & 0xFF) as u8,
                            ]),
                        ),
                    ),
                ],
            );
            sensor.quickstart().unwrap();
            destroy_44(sensor);
        }
    }
    mod max17048 {
        use super::*;
        extern crate test;
        #[cfg(test)]
        #[rustc_test_marker = "common::max17048::can_create_and_destroy"]
        #[doc(hidden)]
        pub const can_create_and_destroy: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("common::max17048::can_create_and_destroy"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "tests/integration.rs",
                start_line: 49usize,
                start_col: 16usize,
                end_line: 49usize,
                end_col: 38usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::IntegrationTest,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(can_create_and_destroy()),
            ),
        };
        fn can_create_and_destroy() {
            let sensor = new_48(&[]);
            destroy_48(sensor);
        }
        extern crate test;
        #[cfg(test)]
        #[rustc_test_marker = "common::max17048::can_get_version"]
        #[doc(hidden)]
        pub const can_get_version: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("common::max17048::can_get_version"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "tests/integration.rs",
                start_line: 55usize,
                start_col: 16usize,
                end_line: 55usize,
                end_col: 31usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::IntegrationTest,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(can_get_version()),
            ),
        };
        fn can_get_version() {
            let version = 0xABCD;
            let mut sensor = new_48(
                &[
                    I2cTrans::write_read(
                        ADDR,
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([Register::VERSION]),
                        ),
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([0xAB, 0xCD]),
                        ),
                    ),
                ],
            );
            let v = sensor.version().unwrap();
            match (&v, &version) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            destroy_48(sensor);
        }
        extern crate test;
        #[cfg(test)]
        #[rustc_test_marker = "common::max17048::quickstart"]
        #[doc(hidden)]
        pub const quickstart: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("common::max17048::quickstart"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "tests/integration.rs",
                start_line: 67usize,
                start_col: 23usize,
                end_line: 67usize,
                end_col: 33usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::IntegrationTest,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(quickstart()),
            ),
        };
        fn quickstart() {
            let mut sensor = new_48(
                &[
                    I2cTrans::write(
                        ADDR,
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                Register::MODE,
                                ((Command::QSTRT & 0xFF00) >> 8) as u8,
                                (Command::QSTRT & 0xFF) as u8,
                            ]),
                        ),
                    ),
                ],
            );
            sensor.quickstart().unwrap();
            destroy_48(sensor);
        }
    }
    mod max17049 {
        use super::*;
        extern crate test;
        #[cfg(test)]
        #[rustc_test_marker = "common::max17049::can_create_and_destroy"]
        #[doc(hidden)]
        pub const can_create_and_destroy: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("common::max17049::can_create_and_destroy"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "tests/integration.rs",
                start_line: 49usize,
                start_col: 16usize,
                end_line: 49usize,
                end_col: 38usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::IntegrationTest,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(can_create_and_destroy()),
            ),
        };
        fn can_create_and_destroy() {
            let sensor = new_49(&[]);
            destroy_49(sensor);
        }
        extern crate test;
        #[cfg(test)]
        #[rustc_test_marker = "common::max17049::can_get_version"]
        #[doc(hidden)]
        pub const can_get_version: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("common::max17049::can_get_version"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "tests/integration.rs",
                start_line: 55usize,
                start_col: 16usize,
                end_line: 55usize,
                end_col: 31usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::IntegrationTest,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(can_get_version()),
            ),
        };
        fn can_get_version() {
            let version = 0xABCD;
            let mut sensor = new_49(
                &[
                    I2cTrans::write_read(
                        ADDR,
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([Register::VERSION]),
                        ),
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([0xAB, 0xCD]),
                        ),
                    ),
                ],
            );
            let v = sensor.version().unwrap();
            match (&v, &version) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            destroy_49(sensor);
        }
        extern crate test;
        #[cfg(test)]
        #[rustc_test_marker = "common::max17049::quickstart"]
        #[doc(hidden)]
        pub const quickstart: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("common::max17049::quickstart"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "tests/integration.rs",
                start_line: 67usize,
                start_col: 23usize,
                end_line: 67usize,
                end_col: 33usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::IntegrationTest,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(quickstart()),
            ),
        };
        fn quickstart() {
            let mut sensor = new_49(
                &[
                    I2cTrans::write(
                        ADDR,
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                Register::MODE,
                                ((Command::QSTRT & 0xFF00) >> 8) as u8,
                                (Command::QSTRT & 0xFF) as u8,
                            ]),
                        ),
                    ),
                ],
            );
            sensor.quickstart().unwrap();
            destroy_49(sensor);
        }
    }
    mod max17058 {
        use super::*;
        extern crate test;
        #[cfg(test)]
        #[rustc_test_marker = "common::max17058::can_create_and_destroy"]
        #[doc(hidden)]
        pub const can_create_and_destroy: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("common::max17058::can_create_and_destroy"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "tests/integration.rs",
                start_line: 49usize,
                start_col: 16usize,
                end_line: 49usize,
                end_col: 38usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::IntegrationTest,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(can_create_and_destroy()),
            ),
        };
        fn can_create_and_destroy() {
            let sensor = new_58(&[]);
            destroy_58(sensor);
        }
        extern crate test;
        #[cfg(test)]
        #[rustc_test_marker = "common::max17058::can_get_version"]
        #[doc(hidden)]
        pub const can_get_version: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("common::max17058::can_get_version"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "tests/integration.rs",
                start_line: 55usize,
                start_col: 16usize,
                end_line: 55usize,
                end_col: 31usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::IntegrationTest,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(can_get_version()),
            ),
        };
        fn can_get_version() {
            let version = 0xABCD;
            let mut sensor = new_58(
                &[
                    I2cTrans::write_read(
                        ADDR,
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([Register::VERSION]),
                        ),
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([0xAB, 0xCD]),
                        ),
                    ),
                ],
            );
            let v = sensor.version().unwrap();
            match (&v, &version) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            destroy_58(sensor);
        }
        extern crate test;
        #[cfg(test)]
        #[rustc_test_marker = "common::max17058::quickstart"]
        #[doc(hidden)]
        pub const quickstart: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("common::max17058::quickstart"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "tests/integration.rs",
                start_line: 67usize,
                start_col: 23usize,
                end_line: 67usize,
                end_col: 33usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::IntegrationTest,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(quickstart()),
            ),
        };
        fn quickstart() {
            let mut sensor = new_58(
                &[
                    I2cTrans::write(
                        ADDR,
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                Register::MODE,
                                ((Command::QSTRT & 0xFF00) >> 8) as u8,
                                (Command::QSTRT & 0xFF) as u8,
                            ]),
                        ),
                    ),
                ],
            );
            sensor.quickstart().unwrap();
            destroy_58(sensor);
        }
    }
    mod max17059 {
        use super::*;
        extern crate test;
        #[cfg(test)]
        #[rustc_test_marker = "common::max17059::can_create_and_destroy"]
        #[doc(hidden)]
        pub const can_create_and_destroy: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("common::max17059::can_create_and_destroy"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "tests/integration.rs",
                start_line: 49usize,
                start_col: 16usize,
                end_line: 49usize,
                end_col: 38usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::IntegrationTest,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(can_create_and_destroy()),
            ),
        };
        fn can_create_and_destroy() {
            let sensor = new_59(&[]);
            destroy_59(sensor);
        }
        extern crate test;
        #[cfg(test)]
        #[rustc_test_marker = "common::max17059::can_get_version"]
        #[doc(hidden)]
        pub const can_get_version: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("common::max17059::can_get_version"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "tests/integration.rs",
                start_line: 55usize,
                start_col: 16usize,
                end_line: 55usize,
                end_col: 31usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::IntegrationTest,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(can_get_version()),
            ),
        };
        fn can_get_version() {
            let version = 0xABCD;
            let mut sensor = new_59(
                &[
                    I2cTrans::write_read(
                        ADDR,
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([Register::VERSION]),
                        ),
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([0xAB, 0xCD]),
                        ),
                    ),
                ],
            );
            let v = sensor.version().unwrap();
            match (&v, &version) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            destroy_59(sensor);
        }
        extern crate test;
        #[cfg(test)]
        #[rustc_test_marker = "common::max17059::quickstart"]
        #[doc(hidden)]
        pub const quickstart: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("common::max17059::quickstart"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "tests/integration.rs",
                start_line: 67usize,
                start_col: 23usize,
                end_line: 67usize,
                end_col: 33usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::IntegrationTest,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(quickstart()),
            ),
        };
        fn quickstart() {
            let mut sensor = new_59(
                &[
                    I2cTrans::write(
                        ADDR,
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                Register::MODE,
                                ((Command::QSTRT & 0xFF00) >> 8) as u8,
                                (Command::QSTRT & 0xFF) as u8,
                            ]),
                        ),
                    ),
                ],
            );
            sensor.quickstart().unwrap();
            destroy_59(sensor);
        }
    }
}
mod max17043 {
    use super::*;
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "max17043::get_soc"]
    #[doc(hidden)]
    pub const get_soc: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("max17043::get_soc"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/integration.rs",
            start_line: 84usize,
            start_col: 16usize,
            end_line: 84usize,
            end_col: 23usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(get_soc()),
        ),
    };
    fn get_soc() {
        let mut sensor = new_43(
            &[
                I2cTrans::write_read(
                    ADDR,
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([Register::SOC]),
                    ),
                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([56, 151])),
                ),
            ],
        );
        let v = sensor.soc().unwrap();
        if !((v - 0.1) < 56.59) {
            ::core::panicking::panic("assertion failed: (v - 0.1) < 56.59")
        }
        if !((v + 0.1) > 56.59) {
            ::core::panicking::panic("assertion failed: (v + 0.1) > 56.59")
        }
        destroy_43(sensor);
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "max17043::voltage"]
    #[doc(hidden)]
    pub const voltage: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("max17043::voltage"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/integration.rs",
            start_line: 85usize,
            start_col: 16usize,
            end_line: 85usize,
            end_col: 23usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(voltage()),
        ),
    };
    fn voltage() {
        let mut sensor = new_43(
            &[
                I2cTrans::write_read(
                    ADDR,
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([Register::VCELL]),
                    ),
                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([0x87, 0x8F])),
                ),
            ],
        );
        let v = sensor.voltage().unwrap();
        if !((v - 0.1) < 2.71) {
            ::core::panicking::panic("assertion failed: (v - 0.1) < 2.71")
        }
        if !((v + 0.1) > 2.71) {
            ::core::panicking::panic("assertion failed: (v + 0.1) > 2.71")
        }
        destroy_43(sensor);
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "max17043::reset"]
    #[doc(hidden)]
    pub const reset: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("max17043::reset"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/integration.rs",
            start_line: 86usize,
            start_col: 15usize,
            end_line: 86usize,
            end_col: 20usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(reset())),
    };
    fn reset() {
        let mut sensor = new_43(
            &[
                I2cTrans::write(
                    ADDR,
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            Register::COMMAND,
                            ((Command::POR_43_44 & 0xFF00) >> 8) as u8,
                            (Command::POR_43_44 & 0xFF) as u8,
                        ]),
                    ),
                ),
            ],
        );
        sensor.reset().unwrap();
        destroy_43(sensor);
    }
}
mod max17044 {
    use super::*;
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "max17044::get_soc"]
    #[doc(hidden)]
    pub const get_soc: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("max17044::get_soc"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/integration.rs",
            start_line: 91usize,
            start_col: 16usize,
            end_line: 91usize,
            end_col: 23usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(get_soc()),
        ),
    };
    fn get_soc() {
        let mut sensor = new_44(
            &[
                I2cTrans::write_read(
                    ADDR,
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([Register::SOC]),
                    ),
                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([56, 151])),
                ),
            ],
        );
        let v = sensor.soc().unwrap();
        if !((v - 0.1) < 56.59) {
            ::core::panicking::panic("assertion failed: (v - 0.1) < 56.59")
        }
        if !((v + 0.1) > 56.59) {
            ::core::panicking::panic("assertion failed: (v + 0.1) > 56.59")
        }
        destroy_44(sensor);
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "max17044::voltage"]
    #[doc(hidden)]
    pub const voltage: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("max17044::voltage"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/integration.rs",
            start_line: 92usize,
            start_col: 16usize,
            end_line: 92usize,
            end_col: 23usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(voltage()),
        ),
    };
    fn voltage() {
        let mut sensor = new_44(
            &[
                I2cTrans::write_read(
                    ADDR,
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([Register::VCELL]),
                    ),
                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([0x87, 0x8F])),
                ),
            ],
        );
        let v = sensor.voltage().unwrap();
        if !((v - 0.1) < 5.42) {
            ::core::panicking::panic("assertion failed: (v - 0.1) < 5.42")
        }
        if !((v + 0.1) > 5.42) {
            ::core::panicking::panic("assertion failed: (v + 0.1) > 5.42")
        }
        destroy_44(sensor);
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "max17044::reset"]
    #[doc(hidden)]
    pub const reset: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("max17044::reset"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/integration.rs",
            start_line: 93usize,
            start_col: 15usize,
            end_line: 93usize,
            end_col: 20usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(reset())),
    };
    fn reset() {
        let mut sensor = new_44(
            &[
                I2cTrans::write(
                    ADDR,
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            Register::COMMAND,
                            ((Command::POR_43_44 & 0xFF00) >> 8) as u8,
                            (Command::POR_43_44 & 0xFF) as u8,
                        ]),
                    ),
                ),
            ],
        );
        sensor.reset().unwrap();
        destroy_44(sensor);
    }
}
mod max17048 {
    use super::*;
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "max17048::get_soc"]
    #[doc(hidden)]
    pub const get_soc: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("max17048::get_soc"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/integration.rs",
            start_line: 127usize,
            start_col: 16usize,
            end_line: 127usize,
            end_col: 23usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(get_soc()),
        ),
    };
    fn get_soc() {
        let mut sensor = new_48(
            &[
                I2cTrans::write_read(
                    ADDR,
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([Register::SOC]),
                    ),
                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([48, 126])),
                ),
            ],
        );
        let v = sensor.soc().unwrap();
        if !((v - 0.1) < 48.49) {
            ::core::panicking::panic("assertion failed: (v - 0.1) < 48.49")
        }
        if !((v + 0.1) > 48.49) {
            ::core::panicking::panic("assertion failed: (v + 0.1) > 48.49")
        }
        destroy_48(sensor);
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "max17048::voltage"]
    #[doc(hidden)]
    pub const voltage: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("max17048::voltage"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/integration.rs",
            start_line: 128usize,
            start_col: 16usize,
            end_line: 128usize,
            end_col: 23usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(voltage()),
        ),
    };
    fn voltage() {
        let mut sensor = new_48(
            &[
                I2cTrans::write_read(
                    ADDR,
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([Register::VCELL]),
                    ),
                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([0xA4, 0x9F])),
                ),
            ],
        );
        let v = sensor.voltage().unwrap();
        if !((v - 0.1) < 3.29) {
            ::core::panicking::panic("assertion failed: (v - 0.1) < 3.29")
        }
        if !((v + 0.1) > 3.29) {
            ::core::panicking::panic("assertion failed: (v + 0.1) > 3.29")
        }
        destroy_48(sensor);
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "max17048::rate"]
    #[doc(hidden)]
    pub const rate: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("max17048::rate"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/integration.rs",
            start_line: 129usize,
            start_col: 16usize,
            end_line: 129usize,
            end_col: 20usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(rate())),
    };
    fn rate() {
        let mut sensor = new_48(
            &[
                I2cTrans::write_read(
                    ADDR,
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([Register::CRATE]),
                    ),
                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([1, 0x45])),
                ),
            ],
        );
        let v = sensor.charge_rate().unwrap();
        if !((v - 0.1) < 67.6) {
            ::core::panicking::panic("assertion failed: (v - 0.1) < 67.6")
        }
        if !((v + 0.1) > 67.6) {
            ::core::panicking::panic("assertion failed: (v + 0.1) > 67.6")
        }
        destroy_48(sensor);
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "max17048::negative_rate"]
    #[doc(hidden)]
    pub const negative_rate: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("max17048::negative_rate"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/integration.rs",
            start_line: 131usize,
            start_col: 9usize,
            end_line: 131usize,
            end_col: 22usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(negative_rate()),
        ),
    };
    fn negative_rate() {
        let mut sensor = new_48(
            &[
                I2cTrans::write_read(
                    ADDR,
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([Register::CRATE]),
                    ),
                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([0xFE, 0xBB])),
                ),
            ],
        );
        let v = sensor.charge_rate().unwrap();
        if !((v - 0.1) < -67.6) {
            ::core::panicking::panic("assertion failed: (v - 0.1) < -67.6")
        }
        if !((v + 0.1) > -67.6) {
            ::core::panicking::panic("assertion failed: (v + 0.1) > -67.6")
        }
        destroy_48(sensor);
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "max17048::reset"]
    #[doc(hidden)]
    pub const reset: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("max17048::reset"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/integration.rs",
            start_line: 140usize,
            start_col: 15usize,
            end_line: 140usize,
            end_col: 20usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(reset())),
    };
    fn reset() {
        let mut sensor = new_48(
            &[
                I2cTrans::write(
                    ADDR,
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            Register::COMMAND,
                            ((Command::POR_X8_X9 & 0xFF00) >> 8) as u8,
                            (Command::POR_X8_X9 & 0xFF) as u8,
                        ]),
                    ),
                ),
            ],
        );
        sensor.reset().unwrap();
        destroy_48(sensor);
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "max17048::set_table"]
    #[doc(hidden)]
    pub const set_table: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("max17048::set_table"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/integration.rs",
            start_line: 141usize,
            start_col: 21usize,
            end_line: 141usize,
            end_col: 30usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(set_table()),
        ),
    };
    fn set_table() {
        let mut expected: Vec<u8> = [0; 129]
            .iter()
            .enumerate()
            .map(|(i, _)| {
                return i as u8;
            })
            .collect();
        expected[0] = 0x40;
        let mut data = [0; 64];
        for i in 0..data.len() {
            data[i] = (((i * 2 + 1) << 8) | i * 2 + 2) as u16;
        }
        let mut sensor = new_48(
            &[
                I2cTrans::write(
                    ADDR,
                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([0x3F, 0x57])),
                ),
                I2cTrans::write(
                    ADDR,
                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([0x3E, 0x4A])),
                ),
                I2cTrans::write(ADDR, expected),
                I2cTrans::write(
                    ADDR,
                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([0x3F, 0x00])),
                ),
                I2cTrans::write(
                    ADDR,
                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([0x3E, 0x00])),
                ),
            ],
        );
        sensor.set_table(&data).unwrap();
        destroy_48(sensor);
    }
}
mod max17058 {
    use super::*;
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "max17058::get_soc"]
    #[doc(hidden)]
    pub const get_soc: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("max17058::get_soc"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/integration.rs",
            start_line: 146usize,
            start_col: 16usize,
            end_line: 146usize,
            end_col: 23usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(get_soc()),
        ),
    };
    fn get_soc() {
        let mut sensor = new_58(
            &[
                I2cTrans::write_read(
                    ADDR,
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([Register::SOC]),
                    ),
                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([48, 126])),
                ),
            ],
        );
        let v = sensor.soc().unwrap();
        if !((v - 0.1) < 48.49) {
            ::core::panicking::panic("assertion failed: (v - 0.1) < 48.49")
        }
        if !((v + 0.1) > 48.49) {
            ::core::panicking::panic("assertion failed: (v + 0.1) > 48.49")
        }
        destroy_58(sensor);
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "max17058::voltage"]
    #[doc(hidden)]
    pub const voltage: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("max17058::voltage"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/integration.rs",
            start_line: 147usize,
            start_col: 16usize,
            end_line: 147usize,
            end_col: 23usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(voltage()),
        ),
    };
    fn voltage() {
        let mut sensor = new_58(
            &[
                I2cTrans::write_read(
                    ADDR,
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([Register::VCELL]),
                    ),
                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([0xA4, 0x9F])),
                ),
            ],
        );
        let v = sensor.voltage().unwrap();
        if !((v - 0.1) < 3.29) {
            ::core::panicking::panic("assertion failed: (v - 0.1) < 3.29")
        }
        if !((v + 0.1) > 3.29) {
            ::core::panicking::panic("assertion failed: (v + 0.1) > 3.29")
        }
        destroy_58(sensor);
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "max17058::reset"]
    #[doc(hidden)]
    pub const reset: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("max17058::reset"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/integration.rs",
            start_line: 148usize,
            start_col: 15usize,
            end_line: 148usize,
            end_col: 20usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(reset())),
    };
    fn reset() {
        let mut sensor = new_58(
            &[
                I2cTrans::write(
                    ADDR,
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            Register::COMMAND,
                            ((Command::POR_X8_X9 & 0xFF00) >> 8) as u8,
                            (Command::POR_X8_X9 & 0xFF) as u8,
                        ]),
                    ),
                ),
            ],
        );
        sensor.reset().unwrap();
        destroy_58(sensor);
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "max17058::set_table"]
    #[doc(hidden)]
    pub const set_table: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("max17058::set_table"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/integration.rs",
            start_line: 149usize,
            start_col: 21usize,
            end_line: 149usize,
            end_col: 30usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(set_table()),
        ),
    };
    fn set_table() {
        let mut expected: Vec<u8> = [0; 129]
            .iter()
            .enumerate()
            .map(|(i, _)| {
                return i as u8;
            })
            .collect();
        expected[0] = 0x40;
        let mut data = [0; 64];
        for i in 0..data.len() {
            data[i] = (((i * 2 + 1) << 8) | i * 2 + 2) as u16;
        }
        let mut sensor = new_58(
            &[
                I2cTrans::write(
                    ADDR,
                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([0x3F, 0x57])),
                ),
                I2cTrans::write(
                    ADDR,
                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([0x3E, 0x4A])),
                ),
                I2cTrans::write(ADDR, expected),
                I2cTrans::write(
                    ADDR,
                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([0x3F, 0x00])),
                ),
                I2cTrans::write(
                    ADDR,
                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([0x3E, 0x00])),
                ),
            ],
        );
        sensor.set_table(&data).unwrap();
        destroy_58(sensor);
    }
}
mod max17049 {
    use super::*;
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "max17049::get_soc"]
    #[doc(hidden)]
    pub const get_soc: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("max17049::get_soc"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/integration.rs",
            start_line: 154usize,
            start_col: 16usize,
            end_line: 154usize,
            end_col: 23usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(get_soc()),
        ),
    };
    fn get_soc() {
        let mut sensor = new_49(
            &[
                I2cTrans::write_read(
                    ADDR,
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([Register::SOC]),
                    ),
                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([48, 126])),
                ),
            ],
        );
        let v = sensor.soc().unwrap();
        if !((v - 0.1) < 48.49) {
            ::core::panicking::panic("assertion failed: (v - 0.1) < 48.49")
        }
        if !((v + 0.1) > 48.49) {
            ::core::panicking::panic("assertion failed: (v + 0.1) > 48.49")
        }
        destroy_49(sensor);
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "max17049::voltage"]
    #[doc(hidden)]
    pub const voltage: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("max17049::voltage"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/integration.rs",
            start_line: 155usize,
            start_col: 16usize,
            end_line: 155usize,
            end_col: 23usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(voltage()),
        ),
    };
    fn voltage() {
        let mut sensor = new_49(
            &[
                I2cTrans::write_read(
                    ADDR,
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([Register::VCELL]),
                    ),
                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([0xA4, 0x9F])),
                ),
            ],
        );
        let v = sensor.voltage().unwrap();
        if !((v - 0.1) < 6.58) {
            ::core::panicking::panic("assertion failed: (v - 0.1) < 6.58")
        }
        if !((v + 0.1) > 6.58) {
            ::core::panicking::panic("assertion failed: (v + 0.1) > 6.58")
        }
        destroy_49(sensor);
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "max17049::rate"]
    #[doc(hidden)]
    pub const rate: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("max17049::rate"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/integration.rs",
            start_line: 156usize,
            start_col: 16usize,
            end_line: 156usize,
            end_col: 20usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(rate())),
    };
    fn rate() {
        let mut sensor = new_49(
            &[
                I2cTrans::write_read(
                    ADDR,
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([Register::CRATE]),
                    ),
                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([1, 0x45])),
                ),
            ],
        );
        let v = sensor.charge_rate().unwrap();
        if !((v - 0.1) < 67.6) {
            ::core::panicking::panic("assertion failed: (v - 0.1) < 67.6")
        }
        if !((v + 0.1) > 67.6) {
            ::core::panicking::panic("assertion failed: (v + 0.1) > 67.6")
        }
        destroy_49(sensor);
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "max17049::negative_rate"]
    #[doc(hidden)]
    pub const negative_rate: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("max17049::negative_rate"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/integration.rs",
            start_line: 158usize,
            start_col: 9usize,
            end_line: 158usize,
            end_col: 22usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(negative_rate()),
        ),
    };
    fn negative_rate() {
        let mut sensor = new_49(
            &[
                I2cTrans::write_read(
                    ADDR,
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([Register::CRATE]),
                    ),
                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([0xFE, 0xBB])),
                ),
            ],
        );
        let v = sensor.charge_rate().unwrap();
        if !((v - 0.1) < -67.6) {
            ::core::panicking::panic("assertion failed: (v - 0.1) < -67.6")
        }
        if !((v + 0.1) > -67.6) {
            ::core::panicking::panic("assertion failed: (v + 0.1) > -67.6")
        }
        destroy_49(sensor);
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "max17049::reset"]
    #[doc(hidden)]
    pub const reset: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("max17049::reset"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/integration.rs",
            start_line: 167usize,
            start_col: 15usize,
            end_line: 167usize,
            end_col: 20usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(reset())),
    };
    fn reset() {
        let mut sensor = new_49(
            &[
                I2cTrans::write(
                    ADDR,
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            Register::COMMAND,
                            ((Command::POR_X8_X9 & 0xFF00) >> 8) as u8,
                            (Command::POR_X8_X9 & 0xFF) as u8,
                        ]),
                    ),
                ),
            ],
        );
        sensor.reset().unwrap();
        destroy_49(sensor);
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "max17049::set_table"]
    #[doc(hidden)]
    pub const set_table: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("max17049::set_table"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/integration.rs",
            start_line: 168usize,
            start_col: 21usize,
            end_line: 168usize,
            end_col: 30usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(set_table()),
        ),
    };
    fn set_table() {
        let mut expected: Vec<u8> = [0; 129]
            .iter()
            .enumerate()
            .map(|(i, _)| {
                return i as u8;
            })
            .collect();
        expected[0] = 0x40;
        let mut data = [0; 64];
        for i in 0..data.len() {
            data[i] = (((i * 2 + 1) << 8) | i * 2 + 2) as u16;
        }
        let mut sensor = new_49(
            &[
                I2cTrans::write(
                    ADDR,
                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([0x3F, 0x57])),
                ),
                I2cTrans::write(
                    ADDR,
                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([0x3E, 0x4A])),
                ),
                I2cTrans::write(ADDR, expected),
                I2cTrans::write(
                    ADDR,
                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([0x3F, 0x00])),
                ),
                I2cTrans::write(
                    ADDR,
                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([0x3E, 0x00])),
                ),
            ],
        );
        sensor.set_table(&data).unwrap();
        destroy_49(sensor);
    }
}
mod max17059 {
    use super::*;
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "max17059::get_soc"]
    #[doc(hidden)]
    pub const get_soc: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("max17059::get_soc"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/integration.rs",
            start_line: 173usize,
            start_col: 16usize,
            end_line: 173usize,
            end_col: 23usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(get_soc()),
        ),
    };
    fn get_soc() {
        let mut sensor = new_59(
            &[
                I2cTrans::write_read(
                    ADDR,
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([Register::SOC]),
                    ),
                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([48, 126])),
                ),
            ],
        );
        let v = sensor.soc().unwrap();
        if !((v - 0.1) < 48.49) {
            ::core::panicking::panic("assertion failed: (v - 0.1) < 48.49")
        }
        if !((v + 0.1) > 48.49) {
            ::core::panicking::panic("assertion failed: (v + 0.1) > 48.49")
        }
        destroy_59(sensor);
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "max17059::voltage"]
    #[doc(hidden)]
    pub const voltage: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("max17059::voltage"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/integration.rs",
            start_line: 174usize,
            start_col: 16usize,
            end_line: 174usize,
            end_col: 23usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(voltage()),
        ),
    };
    fn voltage() {
        let mut sensor = new_59(
            &[
                I2cTrans::write_read(
                    ADDR,
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([Register::VCELL]),
                    ),
                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([0xA4, 0x9F])),
                ),
            ],
        );
        let v = sensor.voltage().unwrap();
        if !((v - 0.1) < 6.58) {
            ::core::panicking::panic("assertion failed: (v - 0.1) < 6.58")
        }
        if !((v + 0.1) > 6.58) {
            ::core::panicking::panic("assertion failed: (v + 0.1) > 6.58")
        }
        destroy_59(sensor);
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "max17059::reset"]
    #[doc(hidden)]
    pub const reset: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("max17059::reset"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/integration.rs",
            start_line: 175usize,
            start_col: 15usize,
            end_line: 175usize,
            end_col: 20usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(reset())),
    };
    fn reset() {
        let mut sensor = new_59(
            &[
                I2cTrans::write(
                    ADDR,
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            Register::COMMAND,
                            ((Command::POR_X8_X9 & 0xFF00) >> 8) as u8,
                            (Command::POR_X8_X9 & 0xFF) as u8,
                        ]),
                    ),
                ),
            ],
        );
        sensor.reset().unwrap();
        destroy_59(sensor);
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "max17059::set_table"]
    #[doc(hidden)]
    pub const set_table: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("max17059::set_table"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/integration.rs",
            start_line: 176usize,
            start_col: 21usize,
            end_line: 176usize,
            end_col: 30usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(set_table()),
        ),
    };
    fn set_table() {
        let mut expected: Vec<u8> = [0; 129]
            .iter()
            .enumerate()
            .map(|(i, _)| {
                return i as u8;
            })
            .collect();
        expected[0] = 0x40;
        let mut data = [0; 64];
        for i in 0..data.len() {
            data[i] = (((i * 2 + 1) << 8) | i * 2 + 2) as u16;
        }
        let mut sensor = new_59(
            &[
                I2cTrans::write(
                    ADDR,
                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([0x3F, 0x57])),
                ),
                I2cTrans::write(
                    ADDR,
                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([0x3E, 0x4A])),
                ),
                I2cTrans::write(ADDR, expected),
                I2cTrans::write(
                    ADDR,
                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([0x3F, 0x00])),
                ),
                I2cTrans::write(
                    ADDR,
                    <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([0x3E, 0x00])),
                ),
            ],
        );
        sensor.set_table(&data).unwrap();
        destroy_59(sensor);
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(
        &[
            &can_create_and_destroy,
            &can_get_version,
            &quickstart,
            &can_create_and_destroy,
            &can_get_version,
            &quickstart,
            &can_create_and_destroy,
            &can_get_version,
            &quickstart,
            &can_create_and_destroy,
            &can_get_version,
            &quickstart,
            &can_create_and_destroy,
            &can_get_version,
            &quickstart,
            &can_create_and_destroy,
            &can_get_version,
            &quickstart,
            &get_soc,
            &reset,
            &voltage,
            &get_soc,
            &reset,
            &voltage,
            &get_soc,
            &negative_rate,
            &rate,
            &reset,
            &set_table,
            &voltage,
            &get_soc,
            &negative_rate,
            &rate,
            &reset,
            &set_table,
            &voltage,
            &get_soc,
            &reset,
            &set_table,
            &voltage,
            &get_soc,
            &reset,
            &set_table,
            &voltage,
        ],
    )
}
