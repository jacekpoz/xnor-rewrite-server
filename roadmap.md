# the roadmap
- postgres setup, issues: 
    - vm: 
        - less clutter on laptop
        - harder development: 
            - have to ssh into it
            - have to setup nvim the same way
            - have to compile there
    - laptop: 
        - way easier development, although
        - have to remove the packages and envvars when I'm done 
          (look into .env files and / or trying nix??)
        - won't know about any connectivity issues since it's all local
        - have to setup everything correctly, 
          otherwise can't compile and have to compile in vm
when postgres is done:
- make an actual API for this thing
- make a database and shit
- do all the routing and connect up to db
- should have an mvp by that point but then also gotta think about
- user login / authentication (preferably keypairs)
- if keypairs handling spam, mass account creation
- ‼️ federation‼️ 
- actual builtin moderation tools
- good bot creator, guis for the simple shit
- native bridging somehow
- maybe make it into a client for multiple platforms??
