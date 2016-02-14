var searchIndex = {};
searchIndex['rust_cache'] = {"items":[[0,"","rust_cache","",null,null],[0,"common","","",null,null],[3,"CacheEntry","rust_cache::common","",null,null],[12,"string","","",0,null],[12,"expires","","",0,null],[3,"NullCache","","",null,null],[4,"ParseCacheEntryError","","",null,null],[13,"NotEnoughParts","","",1,null],[13,"TimespecParseError","","",1,null],[8,"Cacheable","","",null,null],[16,"Error","","",2,null],[10,"to_cache","","",2,{"inputs":[{"name":"cacheable"}],"output":{"name":"result"}}],[10,"from_cache","","",2,{"inputs":[{"name":"cacheable"},{"name":"string"}],"output":{"name":"result"}}],[8,"Cache","","",null,null],[16,"Error","","",3,null],[10,"fetch","","",3,{"inputs":[{"name":"cache"},{"name":"string"}],"output":{"name":"result"}}],[10,"save","","",3,{"inputs":[{"name":"cache"},{"name":"string"},{"name":"t"},{"name":"duration"}],"output":{"name":"result"}}],[10,"delete","","",3,{"inputs":[{"name":"cache"},{"name":"string"}],"output":{"name":"result"}}],[10,"clear","","",3,{"inputs":[{"name":"cache"}],"output":{"name":"result"}}],[11,"eq","","",0,{"inputs":[{"name":"cacheentry"},{"name":"cacheentry"}],"output":{"name":"bool"}}],[11,"ne","","",0,{"inputs":[{"name":"cacheentry"},{"name":"cacheentry"}],"output":{"name":"bool"}}],[11,"fmt","","",0,{"inputs":[{"name":"cacheentry"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"new","","",0,{"inputs":[{"name":"cacheentry"},{"name":"string"},{"name":"duration"}],"output":{"name":"self"}}],[11,"expired","","",0,{"inputs":[{"name":"cacheentry"}],"output":{"name":"bool"}}],[11,"to_string","","",0,{"inputs":[{"name":"cacheentry"}],"output":{"name":"string"}}],[11,"from_str","","",0,{"inputs":[{"name":"cacheentry"},{"name":"str"}],"output":{"name":"result"}}],[11,"eq","","",1,{"inputs":[{"name":"parsecacheentryerror"},{"name":"parsecacheentryerror"}],"output":{"name":"bool"}}],[11,"ne","","",1,{"inputs":[{"name":"parsecacheentryerror"},{"name":"parsecacheentryerror"}],"output":{"name":"bool"}}],[11,"fmt","","",1,{"inputs":[{"name":"parsecacheentryerror"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from","","",1,{"inputs":[{"name":"parsecacheentryerror"},{"name":"parseinterror"}],"output":{"name":"self"}}],[11,"fetch","","",4,{"inputs":[{"name":"nullcache"},{"name":"string"}],"output":{"name":"result"}}],[11,"save","","",4,{"inputs":[{"name":"nullcache"},{"name":"string"},{"name":"t"},{"name":"duration"}],"output":{"name":"result"}}],[11,"delete","","",4,{"inputs":[{"name":"nullcache"},{"name":"string"}],"output":{"name":"result"}}],[11,"clear","","",4,{"inputs":[{"name":"nullcache"}],"output":{"name":"result"}}],[0,"hash_map","rust_cache","",null,null],[3,"HashMapCache","rust_cache::hash_map","",null,null],[11,"new","","",5,{"inputs":[{"name":"hashmapcache"}],"output":{"name":"self"}}],[11,"fetch","","",5,{"inputs":[{"name":"hashmapcache"},{"name":"string"}],"output":{"name":"result"}}],[11,"save","","",5,{"inputs":[{"name":"hashmapcache"},{"name":"string"},{"name":"t"},{"name":"duration"}],"output":{"name":"result"}}],[11,"delete","","",5,{"inputs":[{"name":"hashmapcache"},{"name":"string"}],"output":{"name":"result"}}],[11,"clear","","",5,{"inputs":[{"name":"hashmapcache"}],"output":{"name":"result"}}],[0,"filesystem","rust_cache","",null,null],[3,"FilesystemCache","rust_cache::filesystem","",null,null],[4,"Error","","",null,null],[13,"ExistsButIsNotDirectory","","",6,null],[13,"DirectoryNotWritable","","",6,null],[13,"Io","","",6,null],[13,"CacheEntryFailedToParse","","",6,null],[13,"CacheSerializationFailure","","",6,null],[11,"fmt","","",6,{"inputs":[{"name":"error"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from","","",6,{"inputs":[{"name":"error"},{"name":"error"}],"output":{"name":"self"}}],[11,"from","","",6,{"inputs":[{"name":"error"},{"name":"parsecacheentryerror"}],"output":{"name":"self"}}],[11,"new","","",7,{"inputs":[{"name":"filesystemcache"},{"name":"string"}],"output":{"name":"result"}}],[11,"new_with_extension","","",7,{"inputs":[{"name":"filesystemcache"},{"name":"string"},{"name":"string"}],"output":{"name":"result"}}],[11,"new_with_extension_and_umask","","",7,{"inputs":[{"name":"filesystemcache"},{"name":"string"},{"name":"string"},{"name":"u16"}],"output":{"name":"result"}}],[11,"fetch","","",7,{"inputs":[{"name":"filesystemcache"},{"name":"string"}],"output":{"name":"result"}}],[11,"save","","",7,{"inputs":[{"name":"filesystemcache"},{"name":"string"},{"name":"t"},{"name":"duration"}],"output":{"name":"result"}}],[11,"delete","","",7,{"inputs":[{"name":"filesystemcache"},{"name":"string"}],"output":{"name":"result"}}],[11,"clear","","",7,{"inputs":[{"name":"filesystemcache"}],"output":{"name":"result"}}],[0,"redis","rust_cache","",null,null],[3,"RedisCache","rust_cache::redis","",null,null],[4,"Error","","",null,null],[13,"CacheSerializationFailure","","",8,null],[13,"RedisError","","",8,null],[11,"fmt","","",8,{"inputs":[{"name":"error"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from","","",8,{"inputs":[{"name":"error"},{"name":"rediserror"}],"output":{"name":"self"}}],[11,"new","","",9,{"inputs":[{"name":"rediscache"},{"name":"connection"}],"output":{"name":"rediscache"}}],[11,"fetch","","",9,{"inputs":[{"name":"rediscache"},{"name":"string"}],"output":{"name":"result"}}],[11,"save","","",9,{"inputs":[{"name":"rediscache"},{"name":"string"},{"name":"t"},{"name":"duration"}],"output":{"name":"result"}}],[11,"delete","","",9,{"inputs":[{"name":"rediscache"},{"name":"string"}],"output":{"name":"result"}}],[11,"clear","","",9,{"inputs":[{"name":"rediscache"}],"output":{"name":"result"}}]],"paths":[[3,"CacheEntry"],[4,"ParseCacheEntryError"],[8,"Cacheable"],[8,"Cache"],[3,"NullCache"],[3,"HashMapCache"],[4,"Error"],[3,"FilesystemCache"],[4,"Error"],[3,"RedisCache"]]};
initSearch(searchIndex);
