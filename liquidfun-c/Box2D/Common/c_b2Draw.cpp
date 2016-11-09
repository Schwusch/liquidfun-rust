#include <Box2D/Box2D.h>
#include "c_b2Draw.h"

extern "C" {

	void b2Draw_SetFlags(b2Draw* self, uint32 flags) {
		self->SetFlags(flags);
	}

	uint32 b2Draw_GetFlags(const b2Draw* self) {
		return self->GetFlags();
	}

	void b2Draw_AppendFlags(b2Draw* self, uint32 flags) {
		self->AppendFlags(flags);
	}

	void b2Draw_ClearFlags(b2Draw* self, uint32 flags) {
		self->ClearFlags(flags);
	}

	void b2Draw_DrawPolygon(b2Draw* self, const b2Vec2* vertices, int32 vertexCount, const b2Color& color) {
		self->DrawPolygon(vertices, vertexCount, color);
	}

	void b2Draw_DrawSolidPolygon(b2Draw* self, const b2Vec2* vertices, int32 vertexCount, const b2Color& color) {
		self->DrawSolidPolygon(vertices, vertexCount, color);
	}

	void b2Draw_DrawCircle(b2Draw* self, const b2Vec2& center, float32 radius, const b2Color& color) {
		self->DrawCircle(center, radius, color);
	}

	void b2Draw_DrawSolidCircle(b2Draw* self, const b2Vec2& center, float32 radius, const b2Vec2& axis, const b2Color& color) {
		self->DrawSolidCircle(center, radius, axis, color);
	}

	void b2Draw_DrawParticles(b2Draw* self, const b2Vec2 *centers, float32 radius, const b2ParticleColor *colors, int32 count) {
		self->DrawParticles(centers, radius, colors, count);
	}

	void b2Draw_DrawSegment(b2Draw* self, const b2Vec2& p1, const b2Vec2& p2, const b2Color& color) {
		self->DrawSegment(p1, p2, color);
	}

	void b2Draw_DrawTransform(b2Draw* self, const b2Transform& xf) {
		self->DrawTransform(xf);
	}

} // extern C

