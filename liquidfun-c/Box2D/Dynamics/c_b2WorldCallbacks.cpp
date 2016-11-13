#include <Box2D/Box2D.h>
#include "c_b2WorldCallbacks.h"

extern "C" {

	CppContactListener* CppContactListener_new(ContantListenerTrait* self) {
		return new CppContactListener(self);
	}

	void CppContactListener_delete(CppContactListener* self) {
		delete self;
	}

	CppQueryCallback* CppQueryCallback_new(QueryCallbackTrait* self) {
		return new CppQueryCallback(self);
	}

	void CppQueryCallback_delete(CppQueryCallback* self) {
		delete self;
	}

} // extern C
