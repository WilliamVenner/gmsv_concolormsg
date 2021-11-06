#pragma once

#include "rust/cxx.h"
#include "gmsv_concolormsg/src/plugin/mod.rs.h"

#include <iserverplugin.h>
#include <cstdio>
#include <cstring>

class ConColorMsgPlugin : public IServerPluginCallbacks
{
public:
	ConColorMsgPlugin();
	~ConColorMsgPlugin();

	virtual bool 			Load(CreateInterfaceFn, CreateInterfaceFn) { open(); return true; };
	virtual void			Unload(void) { close(); };
	virtual void			Pause(void) {};
	virtual void			UnPause(void) {};
	virtual const char     *GetPluginDescription(void) { return "gmsv_concolormsg"; };
	virtual void			LevelInit(char const *) {};
	virtual void			ServerActivate(void *, int, int) {};
	virtual void			GameFrame(bool) {};
	virtual void			LevelShutdown(void) {};
	virtual void			ClientActive(void *) {};
	virtual void			ClientDisconnect(void *) {};
	virtual void			ClientPutInServer(void *, char const *) {};
	virtual void			SetCommandClient(int ) {};
	virtual void			ClientSettingsChanged(void *) {};
	virtual PLUGIN_RESULT 	ClientConnect(bool *, void *, const char *, const char *, char *, int ) { return PLUGIN_CONTINUE; };
	virtual PLUGIN_RESULT	ClientCommand(void *, const CCommand &) { return PLUGIN_CONTINUE; };
	virtual PLUGIN_RESULT	NetworkIDValidated(const char *, const char *) { return PLUGIN_CONTINUE; };
	virtual void 			OnQueryCvarValueFinished(QueryCvarCookie_t , void *, EQueryCvarValueStatus , const char *, const char *) {};
	virtual void			OnEdictAllocated(void *) {};
	virtual void			OnEdictFreed(const void *) {};
};

uintptr_t CreateInterface();