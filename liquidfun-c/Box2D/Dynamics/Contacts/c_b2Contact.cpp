#include <Box2D/Box2D.h>
// #include "c_b2Contact.h"

extern "C" {

	b2Fixture* b2Contact_GetFixtureA(b2Contact* self) {
		return self->GetFixtureA();
	}

	b2Fixture* b2Contact_GetFixtureB(b2Contact* self) {
		return self->GetFixtureB();
	}

} // extern C

