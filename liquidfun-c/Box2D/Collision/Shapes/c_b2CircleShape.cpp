
extern "C" {

	b2CircleShape* b2CircleShape_New() {
        return new b2CircleShape();
	}

	void b2CircleShape_Delete(b2CircleShape* self) {
		delete self;
	}

    b2Shape* b2CircleShape_Upcast(b2CircleShape* self) {
        return static_cast<b2Shape*>(reinterpret_cast<b2CircleShape*>(self));
    }

	void b2CircleShape_SetPosition(b2CircleShape* self, const b2Vec2& pos) {
		self->m_p = pos;
	}

	void b2CircleShape_SetRadius(b2CircleShape* self, float32 radius) {
		self->m_radius = radius;
	}

} // extern C
