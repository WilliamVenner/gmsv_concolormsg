#include <plugin.h>

ConColorMsgPlugin::ConColorMsgPlugin() {}

static ConColorMsgPlugin* INSTANCE = new ConColorMsgPlugin();
uintptr_t CreateInterface() {
	return (uintptr_t) INSTANCE;
}