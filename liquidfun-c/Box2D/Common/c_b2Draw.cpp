#include <Box2D/Box2D.h>
#include "c_b2Draw.h"

extern "C" {

	CppDebugDraw* CppDebugDraw_new(DrawTrait* self) {
		return new CppDebugDraw(self);
	}

	void CppDebugDraw_delete(CppDebugDraw* self) {
		delete self;
	}

	void CppDebugDraw_SetFlags(CppDebugDraw* self, uint32 flags) {
		return self->SetFlags(flags);
	}

	uint32 CppDebugDraw_GetFlags(CppDebugDraw* self) {
		return self->GetFlags();
	}

	void CppDebugDraw_AppendFlags(CppDebugDraw* self, uint32 flags) {
		return self->AppendFlags(flags);
	}

	void CppDebugDraw_ClearFlags(CppDebugDraw* self, uint32 flags) {
		return self->ClearFlags(flags);
	}

} // extern C
