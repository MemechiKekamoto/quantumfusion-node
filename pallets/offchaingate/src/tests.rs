use crate::{mock::*, Error, Event, Calldata};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(OffchainGateModule::verify_calldata(RuntimeOrigin::signed(1), 42));
		assert_eq!(Calldata::<Test>::get(), Some(42));
		System::assert_last_event(Event::CalldataStored { calldata: 42, who: 1 }.into());
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			OffchainGateModule::cause_error(RuntimeOrigin::signed(1)),
			Error::<Test>::NoneValue
		);
	});
}
