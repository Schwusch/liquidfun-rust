#ifndef C_B2_DRAW
#define C_B2_DRAW

#ifdef __cplusplus
extern "C" {
#endif

	void b2Draw_SetFlags(b2Draw* self, uint32 flags);
	uint32 b2Draw_GetFlags(const b2Draw* self);
	void b2Draw_AppendFlags(b2Draw* self, uint32 flags);
	void b2Draw_ClearFlags(b2Draw* self, uint32 flags);
	void b2Draw_DrawPolygon(b2Draw* self, const b2Vec2* vertices, int32 vertexCount, const b2Color& color);
	void b2Draw_DrawSolidPolygon(b2Draw* self, const b2Vec2* vertices, int32 vertexCount, const b2Color& color);
	void b2Draw_DrawCircle(b2Draw* self, const b2Vec2& center, float32 radius, const b2Color& color);
	void b2Draw_DrawSolidCircle(b2Draw* self, const b2Vec2& center, float32 radius, const b2Vec2& axis, const b2Color& color);
	void b2Draw_DrawParticles(b2Draw* self, const b2Vec2 *centers, float32 radius, const b2ParticleColor *colors, int32 count);
	void b2Draw_DrawSegment(b2Draw* self, const b2Vec2& p1, const b2Vec2& p2, const b2Color& color);
	void b2Draw_DrawTransform(b2Draw* self, const b2Transform& xf);

#ifdef __cplusplus
} // extern C
#endif
#endif