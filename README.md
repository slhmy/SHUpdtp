# Welcome to SHU program design&training platform! 💻
After two month work on [online_judge](https://github.com/slhmy/online_judge). 
I decide to restart the project and lead it to final release.

[online_judge](https://github.com/slhmy/online_judge) can run properly, but there are serveral problem which will cost a lot of time to fix. Such as：
- API standard. I did really a mess, now I think REST is good enough. GraphQL will be an optional demand, which may won't come into use immediately.
- Database connection. I used SyncArbiter to realize async diesel, and this lead to a lot of redundancy because you need to implement actor for every query. And it seems that there is no actor survive after query storm. This time I will try r2d2 manager. A presure test will be done recently.
- JudgeServer connection. Preemptive workings among actors are not elegant. Logically, you should be able to delegate serval worker to a certain JudgeServer. [JudgeServer from QingdaoU](https://github.com/QingdaoU/JudgeServer) is good, so it will still be in used before our own machine is build.

There is a Matching front-end for [online_judge](https://github.com/slhmy/online_judge). You can see this demo in [ojFront](https://github.com/slhmy/ojFront).

## Target
The biggest topic is to make everything far more simple than [online_judge](https://github.com/slhmy/online_judge).
Basic functions will be refined, so that additional functions can be built more easily. The file structure will be clearer, no dead codes, zero warning and easy to understand.
When everything is well done, then it is the release date.