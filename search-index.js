var searchIndex = {};
searchIndex['rust_cache'] = {"items":[[0,"","rust_cache","",null,null],[0,"common","","",null,null],[3,"CacheEntry","rust_cache::common","This is a useful struct for use inside a cache implentation to keep track\nof the data and the expiration together. it can be converted to a form a string iteself for storage in\nany medium that accepts strings",null,null],[12,"string","","",0,null],[12,"expires","","",0,null],[3,"NullCache","","This sturct is for use in places that require a Cache impl but you dont want to actually\ncache anything. exmples could be testing and such",null,null],[4,"ParseCacheEntryError","","",null,null],[13,"NotEnoughParts","","",1,null],[13,"TimespecParseError","","",1,null],[8,"Cacheable","","",null,null],[16,"Error","","",2,null],[10,"to_cache","","",2,{"inputs":[{"name":"cacheable"}],"output":{"name":"result"}}],[10,"from_cache","","",2,{"inputs":[{"name":"cacheable"},{"name":"string"}],"output":{"name":"result"}}],[8,"Cache","","Trait to implement for actual cache implementations",null,null],[16,"Error","","",3,null],[10,"fetch","","",3,{"inputs":[{"name":"cache"},{"name":"string"}],"output":{"name":"result"}}],[10,"save","","",3,{"inputs":[{"name":"cache"},{"name":"string"},{"name":"t"},{"name":"duration"}],"output":{"name":"result"}}],[10,"delete","","",3,{"inputs":[{"name":"cache"},{"name":"string"}],"output":{"name":"result"}}],[10,"clear","","",3,{"inputs":[{"name":"cache"}],"output":{"name":"result"}}],[11,"eq","","",0,{"inputs":[{"name":"cacheentry"},{"name":"cacheentry"}],"output":{"name":"bool"}}],[11,"ne","","",0,{"inputs":[{"name":"cacheentry"},{"name":"cacheentry"}],"output":{"name":"bool"}}],[11,"fmt","","",0,{"inputs":[{"name":"cacheentry"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"new","","",0,{"inputs":[{"name":"cacheentry"},{"name":"string"},{"name":"duration"}],"output":{"name":"self"}}],[11,"expired","","",0,{"inputs":[{"name":"cacheentry"}],"output":{"name":"bool"}}],[11,"to_string","","",0,{"inputs":[{"name":"cacheentry"}],"output":{"name":"string"}}],[11,"from_str","","",0,{"inputs":[{"name":"cacheentry"},{"name":"str"}],"output":{"name":"result"}}],[11,"eq","","",1,{"inputs":[{"name":"parsecacheentryerror"},{"name":"parsecacheentryerror"}],"output":{"name":"bool"}}],[11,"ne","","",1,{"inputs":[{"name":"parsecacheentryerror"},{"name":"parsecacheentryerror"}],"output":{"name":"bool"}}],[11,"fmt","","",1,{"inputs":[{"name":"parsecacheentryerror"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from","","",1,{"inputs":[{"name":"parsecacheentryerror"},{"name":"parseinterror"}],"output":{"name":"self"}}],[11,"fetch","","",4,{"inputs":[{"name":"nullcache"},{"name":"string"}],"output":{"name":"result"}}],[11,"save","","",4,{"inputs":[{"name":"nullcache"},{"name":"string"},{"name":"t"},{"name":"duration"}],"output":{"name":"result"}}],[11,"delete","","",4,{"inputs":[{"name":"nullcache"},{"name":"string"}],"output":{"name":"result"}}],[11,"clear","","",4,{"inputs":[{"name":"nullcache"}],"output":{"name":"result"}}],[0,"hash_map","rust_cache","",null,null],[3,"HashMapCache","rust_cache::hash_map","",null,null],[4,"Error","","",null,null],[13,"CacheSerializationFailure","","",5,null],[11,"fmt","","",5,{"inputs":[{"name":"error"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"new","","",6,{"inputs":[{"name":"hashmapcache"}],"output":{"name":"self"}}],[11,"fetch","","",6,{"inputs":[{"name":"hashmapcache"},{"name":"string"}],"output":{"name":"result"}}],[11,"save","","",6,{"inputs":[{"name":"hashmapcache"},{"name":"string"},{"name":"t"},{"name":"duration"}],"output":{"name":"result"}}],[11,"delete","","",6,{"inputs":[{"name":"hashmapcache"},{"name":"string"}],"output":{"name":"result"}}],[11,"clear","","",6,{"inputs":[{"name":"hashmapcache"}],"output":{"name":"result"}}]],"paths":[[3,"CacheEntry"],[4,"ParseCacheEntryError"],[8,"Cacheable"],[8,"Cache"],[3,"NullCache"],[4,"Error"],[3,"HashMapCache"]]};
initSearch(searchIndex);
