use crate::prelude::LinearKeyframe;

#[test]
fn linear_derive_struct() {
    #[derive(LinearKeyframe, Clone, PartialEq)]
    struct Test {
        name: u32,
        other: f32,
    }

    assert!(
        Test {
            name: 0,
            other: 0.0,
        }
        .lerp(
            &Test {
                name: 10,
                other: 10.0,
            },
            0.5
        ) == Test {
            name: 5,
            other: 5.0,
        }
    );
    assert!(
        Test {
            name: 0,
            other: 0.0,
        }
        .lerp(
            &Test {
                name: 10,
                other: 10.0,
            },
            0.75
        ) == Test {
            name: 7,
            other: 7.5,
        }
    );
    assert!(
        Test {
            name: 0,
            other: 0.0,
        }
        .lerp(
            &Test {
                name: 10,
                other: 10.0,
            },
            0.25
        ) == Test {
            name: 2,
            other: 2.5,
        }
    )
}
