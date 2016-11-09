#ifndef C_B2_DRAW
#define C_B2_DRAW

struct BoxDebugDraw {};

#ifdef __cplusplus
extern "C" {
#endif

	void BoxDebugDraw_SetFlags(BoxDebugDraw* self, uint32 flags);
	uint32 BoxDebugDraw_GetFlags(const BoxDebugDraw* self);
	void BoxDebugDraw_AppendFlags(BoxDebugDraw* self, uint32 flags);
	void BoxDebugDraw_ClearFlags(BoxDebugDraw* self, uint32 flags);
	void BoxDebugDraw_DrawPolygon(BoxDebugDraw* self, const b2Vec2* vertices, int32 vertexCount, const b2Color& color);
	void BoxDebugDraw_DrawSolidPolygon(BoxDebugDraw* self, const b2Vec2* vertices, int32 vertexCount, const b2Color& color);
	void BoxDebugDraw_DrawCircle(BoxDebugDraw* self, const b2Vec2& center, float32 radius, const b2Color& color);
	void BoxDebugDraw_DrawSolidCircle(BoxDebugDraw* self, const b2Vec2& center, float32 radius, const b2Vec2& axis, const b2Color& color);
	void BoxDebugDraw_DrawParticles(BoxDebugDraw* self, const b2Vec2 *centers, float32 radius, const b2ParticleColor *colors, int32 count);
	void BoxDebugDraw_DrawSegment(BoxDebugDraw* self, const b2Vec2& p1, const b2Vec2& p2, const b2Color& color);
	void BoxDebugDraw_DrawTransform(BoxDebugDraw* self, const b2Transform& xf);

#ifdef __cplusplus
} // extern C
#endif

class CppDebugDraw: b2Draw {
public:
	CppDebugDraw(BoxDebugDraw* debugDraw): self(debugDraw) {}

	void SetFlags(uint32 flags) {
		BoxDebugDraw_SetFlags(self, flags);
	}

	uint32 GetFlags() const {
		return BoxDebugDraw_GetFlags(self);
	}

	void AppendFlags(uint32 flags) {
		BoxDebugDraw_AppendFlags(self, flags);
	}

	void ClearFlags(uint32 flags) {
		BoxDebugDraw_ClearFlags(self, flags);
	}

	virtual void DrawPolygon(const b2Vec2* vertices, int32 vertexCount, const b2Color& color) {
		BoxDebugDraw_DrawPolygon(self, vertices, vertexCount, color);
	}

	virtual void DrawSolidPolygon(const b2Vec2* vertices, int32 vertexCount, const b2Color& color) {
		BoxDebugDraw_DrawSolidPolygon(self, vertices, vertexCount, color);
	}

	virtual void DrawCircle(const b2Vec2& center, float32 radius, const b2Color& color) {
		BoxDebugDraw_DrawCircle(self, center, radius, color);
	}

	virtual void DrawSolidCircle(const b2Vec2& center, float32 radius, const b2Vec2& axis, const b2Color& color) {
		BoxDebugDraw_DrawSolidCircle(self, center, radius, axis, color);
	}

	virtual void DrawParticles(const b2Vec2 *centers, float32 radius, const b2ParticleColor *colors, int32 count) {
		BoxDebugDraw_DrawParticles(self, centers, radius, colors, count);
	}

	virtual void DrawSegment(const b2Vec2& p1, const b2Vec2& p2, const b2Color& color) {
		BoxDebugDraw_DrawSegment(self, p1, p2, color);
	}

	virtual void DrawTransform(const b2Transform& xf) {
		BoxDebugDraw_DrawTransform(self, xf);
	}

protected:
	BoxDebugDraw* self;
};

#ifdef __cplusplus
extern "C" {
#endif

	CppDebugDraw* DebugDraw_new(BoxDebugDraw* debugDraw);

#ifdef __cplusplus
} // extern C
#endif

#endif