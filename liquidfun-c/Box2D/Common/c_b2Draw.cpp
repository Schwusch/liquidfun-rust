#include <Box2D/Box2D.h>
#include "c_b2Draw.h"

extern "C" {

	CppDebugDraw* DebugDraw_new(BoxDebugDraw* debugDraw) {
		return new CppDebugDraw(debugDraw);
	}

} // extern C
