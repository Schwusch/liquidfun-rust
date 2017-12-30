#include <Box2D/Box2D.h>
#include "c_b2Body.h"

extern "C" {

	b2Fixture* b2Body_CreateFixture(b2Body* self, const b2FixtureDef* def) {
		return self->CreateFixture(def);
	}

	b2Fixture* b2Body_CreateFixture_FromShape(b2Body* self, const b2Shape* shape, float32 density) {
		return self->CreateFixture(shape, density);
	}
	
	float32 b2Body_GetAngle(const b2Body* self) {
		return self->GetAngle();
	}

	b2Fixture* b2Body_GetFixtureList(b2Body* self) {
		return self->GetFixtureList();
	}

	b2Body* b2Body_GetNext(b2Body* self) {
		return self->GetNext();
	}

	const b2Vec2& b2Body_GetPosition(const b2Body* self) {
		return self->GetPosition();
	}

	void* b2Body_GetUserData(const b2Body* self) {
		return self->GetUserData();
	}

	void b2Body_SetUserData(b2Body* self, void* data) {
		return self->SetUserData(data);
	}

	b2World* b2Body_GetWorld(b2Body* self) {
		return self->GetWorld();
	}

	b2Vec2 b2Body_GetLocalPoint(const b2Body* self, const b2Vec2& worldPoint) {
		return self->GetLocalPoint(worldPoint);
	}

	void b2Body_SetTransform(b2Body* self, const b2Vec2& position, float32 angle) {
		self->SetTransform(position, angle);
	}

	void b2Body_SetLinearVelocity(b2Body* self, const b2Vec2& v) {
		self->SetLinearVelocity(v);
	}

	const b2Vec2& b2Body_GetLinearVelocity(const b2Body* self) {
		return self->GetLinearVelocity();
	}

	void b2Body_ApplyForceToCenter(b2Body* self, const b2Vec2& force, bool wake) {
		self->ApplyForceToCenter(force, wake);
	}

	void b2Body_ApplyLinearImpulse(b2Body* self, const b2Vec2& impulse, const b2Vec2& point, bool wake) {
		self->ApplyLinearImpulse(impulse, point, wake);
	}

	const b2Vec2& b2Body_GetWorldCenter(const b2Body* self) {
		return self->GetWorldCenter();
	}

	const b2Vec2& b2Body_GetLocalCenter(const b2Body* self) {
		return self->GetLocalCenter();
	}

	void b2Body_SetLinearDamping(b2Body* self, float32 linearDamping) {
		self->SetLinearDamping(linearDamping);
	}

	void b2Body_SetFixedRotation(b2Body* self, bool flag) {
		self->SetFixedRotation(flag);
	}

	float32 b2Body_GetMass(const b2Body* self) {
		return self->GetMass();
	}

	void b2Body_DestroyFixture(b2Body* self, b2Fixture* fixture) {
		self->DestroyFixture(fixture);
	}
	void b2Body_ApplyForce(b2Body* self, const b2Vec2& force, const b2Vec2& point, bool wake) {
        self->ApplyForce(force, point, wake);
    }

} // extern C

