#ifndef C_B2_DRAW
#define C_B2_DRAW

struct DrawTrait {};

#ifdef __cplusplus
extern "C" {
#endif

	void DrawTrait_DrawPolygon(DrawTrait* self, const b2Vec2* vertices, int32 vertexCount, const b2Color& color);
	void DrawTrait_DrawSolidPolygon(DrawTrait* self, const b2Vec2* vertices, int32 vertexCount, const b2Color& color);
	void DrawTrait_DrawCircle(DrawTrait* self, const b2Vec2& center, float32 radius, const b2Color& color);
	void DrawTrait_DrawSolidCircle(DrawTrait* self, const b2Vec2& center, float32 radius, const b2Vec2& axis, const b2Color& color);
	void DrawTrait_DrawParticles(DrawTrait* self, const b2Vec2 *centers, float32 radius, const b2ParticleColor *colors, int32 count);
	void DrawTrait_DrawSegment(DrawTrait* self, const b2Vec2& p1, const b2Vec2& p2, const b2Color& color);
	void DrawTrait_DrawTransform(DrawTrait* self, const b2Transform& xf);

#ifdef __cplusplus
} // extern C
#endif

class CppDebugDraw: public b2Draw {
public:
	CppDebugDraw(DrawTrait* debugDraw): self(debugDraw) {}

	virtual void DrawPolygon(const b2Vec2* vertices, int32 vertexCount, const b2Color& color) {
		DrawTrait_DrawPolygon(self, vertices, vertexCount, color);
	}

	virtual void DrawSolidPolygon(const b2Vec2* vertices, int32 vertexCount, const b2Color& color) {
		DrawTrait_DrawSolidPolygon(self, vertices, vertexCount, color);
	}

	virtual void DrawCircle(const b2Vec2& center, float32 radius, const b2Color& color) {
		DrawTrait_DrawCircle(self, center, radius, color);
	}

	virtual void DrawSolidCircle(const b2Vec2& center, float32 radius, const b2Vec2& axis, const b2Color& color) {
		DrawTrait_DrawSolidCircle(self, center, radius, axis, color);
	}

	virtual void DrawParticles(const b2Vec2 *centers, float32 radius, const b2ParticleColor *colors, int32 count) {
		DrawTrait_DrawParticles(self, centers, radius, colors, count);
	}

	virtual void DrawSegment(const b2Vec2& p1, const b2Vec2& p2, const b2Color& color) {
		DrawTrait_DrawSegment(self, p1, p2, color);
	}

	virtual void DrawTransform(const b2Transform& xf) {
		DrawTrait_DrawTransform(self, xf);
	}

protected:
	DrawTrait* self;
};

#ifdef __cplusplus
extern "C" {
#endif

	CppDebugDraw* CppDebugDraw_new(DrawTrait* self);
	void CppDebugDraw_delete(CppDebugDraw* self);
	void CppDebugDraw_SetFlags(CppDebugDraw* self, uint32 flags);
	uint32 CppDebugDraw_GetFlags(CppDebugDraw* self);
	void CppDebugDraw_AppendFlags(CppDebugDraw* self, uint32 flags);
	void CppDebugDraw_ClearFlags(CppDebugDraw* self, uint32 flags);

#ifdef __cplusplus
} // extern C
#endif

#endif
