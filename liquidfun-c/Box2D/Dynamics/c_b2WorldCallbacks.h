#ifndef C_B2_WORLD_CALLBACKS_H
#define C_B2_WORLD_CALLBACKS_H

struct ContantListenerTrait {};
struct QueryCallbackTrait {};

#ifdef __cplusplus
extern "C" {
#endif

	void ContantListenerTrait_BeginContact(ContantListenerTrait* self, b2Contact* contact);
	void ContantListenerTrait_EndContact(ContantListenerTrait* self, b2Contact* contact);
	bool QueryCallbackTrait_ReportFixture(QueryCallbackTrait* self, b2Fixture* fixture);

#ifdef __cplusplus
} // extern C
#endif

class CppContactListener : public b2ContactListener {
public:

	CppContactListener(ContantListenerTrait* self): self(self) {}

	/// Called when two fixtures begin to touch.
	virtual void BeginContact(b2Contact* contact) {
		ContantListenerTrait_BeginContact(self, contact);
	}

	/// Called when two fixtures cease to touch.
	virtual void EndContact(b2Contact* contact) {
		ContantListenerTrait_EndContact(self, contact);
	}

	/// Called when a fixture and particle start touching if the
	/// b2_fixtureContactFilterParticle flag is set on the particle.
	virtual void BeginContact(b2ParticleSystem* particleSystem,
							  b2ParticleBodyContact* particleBodyContact)
	{
		B2_NOT_USED(particleSystem);
		B2_NOT_USED(particleBodyContact);
	}

	/// Called when a fixture and particle stop touching if the
	/// b2_fixtureContactFilterParticle flag is set on the particle.
	virtual void EndContact(b2Fixture* fixture,
							b2ParticleSystem* particleSystem, int32 index)
	{
		B2_NOT_USED(fixture);
		B2_NOT_USED(particleSystem);
		B2_NOT_USED(index);
	}

	/// Called when two particles start touching if
	/// b2_particleContactFilterParticle flag is set on either particle.
	virtual void BeginContact(b2ParticleSystem* particleSystem,
							  b2ParticleContact* particleContact)
	{
		B2_NOT_USED(particleSystem);
		B2_NOT_USED(particleContact);
	}

	/// Called when two particles start touching if
	/// b2_particleContactFilterParticle flag is set on either particle.
	virtual void EndContact(b2ParticleSystem* particleSystem,
							int32 indexA, int32 indexB)
	{
		B2_NOT_USED(particleSystem);
		B2_NOT_USED(indexA);
		B2_NOT_USED(indexB);
	}

	/// This is called after a contact is updated. This allows you to inspect a
	/// contact before it goes to the solver. If you are careful, you can modify the
	/// contact manifold (e.g. disable contact).
	/// A copy of the old manifold is provided so that you can detect changes.
	/// Note: this is called only for awake bodies.
	/// Note: this is called even when the number of contact points is zero.
	/// Note: this is not called for sensors.
	/// Note: if you set the number of contact points to zero, you will not
	/// get an EndContact callback. However, you may get a BeginContact callback
	/// the next step.
	virtual void PreSolve(b2Contact* contact, const b2Manifold* oldManifold)
	{
		B2_NOT_USED(contact);
		B2_NOT_USED(oldManifold);
	}

	/// This lets you inspect a contact after the solver is finished. This is useful
	/// for inspecting impulses.
	/// Note: the contact manifold does not include time of impact impulses, which can be
	/// arbitrarily large if the sub-step is small. Hence the impulse is provided explicitly
	/// in a separate data structure.
	/// Note: this is only called for contacts that are touching, solid, and awake.
	virtual void PostSolve(b2Contact* contact, const b2ContactImpulse* impulse)
	{
		B2_NOT_USED(contact);
		B2_NOT_USED(impulse);
	}

protected:
	ContantListenerTrait* self;
};

class CppQueryCallback : public b2QueryCallback {
public:
	CppQueryCallback(QueryCallbackTrait* self): self(self) {}

	/// Called for each fixture found in the query AABB.
	/// @return false to terminate the query.
	virtual bool ReportFixture(b2Fixture* fixture) {
		return QueryCallbackTrait_ReportFixture(self, fixture);
	}

	/// Called for each particle found in the query AABB.
	/// @return false to terminate the query.
	virtual bool ReportParticle(const b2ParticleSystem* particleSystem,
								int32 index)
	{
		B2_NOT_USED(particleSystem);
		B2_NOT_USED(index);
		return false;
	}

	/// Cull an entire particle system from b2World::QueryAABB. Ignored for
	/// b2ParticleSystem::QueryAABB.
	/// @return true if you want to include particleSystem in the AABB query,
	/// or false to cull particleSystem from the AABB query.
	virtual bool ShouldQueryParticleSystem(
		const b2ParticleSystem* particleSystem)
	{
		B2_NOT_USED(particleSystem);
		return false;
	}

protected:
	QueryCallbackTrait* self;
};

#ifdef __cplusplus
extern "C" {
#endif

	CppContactListener* CppContactListener_new(ContantListenerTrait* self);
	void CppContactListener_delete(CppContactListener* self);
	CppQueryCallback* CppQueryCallback_new(QueryCallbackTrait* self);
	void CppQueryCallback_delete(CppQueryCallback* self);

#ifdef __cplusplus
} // extern C
#endif

#endif
