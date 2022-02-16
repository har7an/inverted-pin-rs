use dummy_pin::LastStateDummyPin;
use embedded_hal::digital::v2::{InputPin, OutputPin, StatefulOutputPin, ToggleableOutputPin};
use embedded_hal_mock::pin::{Mock as PinMock, State, Transaction, TransactionKind};
use inverted_pin::{InvertedInputPin, InvertedOutputPin};

#[test]
fn can_create() {
    let mock_pin = PinMock::new(&[]);
    let pin = InvertedOutputPin::new(mock_pin);
    let mut mock_pin = pin.destroy();
    mock_pin.done();
}

#[test]
fn output_sets_inverted_logic() {
    let mock_pin = PinMock::new(&[
        Transaction::new(TransactionKind::Set(State::Low)),
        Transaction::new(TransactionKind::Set(State::High)),
    ]);
    let mut pin = InvertedOutputPin::new(mock_pin);
    pin.set_high().unwrap();
    pin.set_low().unwrap();
    let mut mock_pin = pin.destroy();
    mock_pin.done();
}

#[test]
fn input_gets_inverted_logic() {
    let mock_pin = PinMock::new(&[
        Transaction::new(TransactionKind::Get(State::Low)),
        Transaction::new(TransactionKind::Get(State::High)),
    ]);
    let pin = InvertedInputPin::new(mock_pin);
    assert!(pin.is_high().unwrap());
    assert!(pin.is_low().unwrap());
    let mut mock_pin = pin.destroy();
    mock_pin.done();
}

// for the rest of tests we use a dummy pin because `embedded-hal-mock` does not support these traits at the moment

#[test]
fn stateful_output_is_set_high_gets_inverted_logic() {
    let mock_pin = LastStateDummyPin::new_high();
    let mut pin = InvertedOutputPin::new(mock_pin);
    pin.set_high().unwrap();
    assert!(pin.is_set_high().unwrap());
    let mock_pin = pin.destroy();
    assert!(mock_pin.is_set_low().unwrap());
}

#[test]
fn stateful_output_is_set_low_gets_inverted_logic() {
    let mock_pin = LastStateDummyPin::new_low();
    let mut pin = InvertedOutputPin::new(mock_pin);
    pin.set_low().unwrap();
    assert!(pin.is_set_low().unwrap());
    let mock_pin = pin.destroy();
    assert!(mock_pin.is_set_high().unwrap());
}

#[test]
fn output_can_toggle() {
    let mock_pin = LastStateDummyPin::new_high();
    let mut pin = InvertedOutputPin::new(mock_pin);
    pin.toggle().unwrap();
    let mock_pin = pin.destroy();
    assert!(mock_pin.is_low().unwrap());
}
