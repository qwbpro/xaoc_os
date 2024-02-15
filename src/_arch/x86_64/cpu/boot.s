
//------------------------------------------------------------------------------
// fn _start()
//------------------------------------------------------------------------------
_start:



	// Prepare the jump to Rust code.
	// Infinitely wait for events (aka "park the core").
	parking_loop:
	
		loop parking_loop

