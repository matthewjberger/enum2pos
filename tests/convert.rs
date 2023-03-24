use enum2pos::EnumIndex;

// Example enum to test the macro
#[derive(EnumIndex, Debug, PartialEq)]
enum SampleEnum {
    Unit,
    Unnamed(i32, String),
    Named { field: u32 },
}

// Test for to_index function
#[test]
fn test_to_index() {
    let unit = SampleEnum::Unit;
    let unnamed = SampleEnum::Unnamed(42, String::from("test"));

    assert_eq!(unit.to_index(), 0);
    assert_eq!(unnamed.to_index(), 1);
}

// Test for from_index function with unit variant
#[test]
fn test_from_index_unit() {
    let index = 0;
    let args: Vec<String> = vec![];
    let expected = Some(SampleEnum::Unit);

    assert_eq!(SampleEnum::from_index(index, args), expected);
}

// Test for from_index function with unnamed variant
#[test]
fn test_from_index_unnamed() {
    let index = 1;
    let args = vec!["42".to_string(), "test".to_string()];
    let expected = Some(SampleEnum::Unnamed(42, String::from("test")));

    assert_eq!(SampleEnum::from_index(index, args), expected);
}

// Test for from_index function with invalid index
#[test]
fn test_from_index_invalid() {
    let index = 2;
    let args: Vec<String> = vec![];

    assert_eq!(SampleEnum::from_index(index, args), None);
}
