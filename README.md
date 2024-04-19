
# NoxV9Datbase

### Welcome to NoxV9Database a database written in rust to handel data from my multi-media database (NoxDatabase)

#### goal of NoxV9Database

* first one is to have a custom datastorage device that I can configure as i please. 
* secound thing I want is a easy way to connect it to my ongoing projects via NoxDatabase.
* third I want it to be fast and a easy to use. 

#### specs

* I will use rust as the language and some kind of clinet server layout to comunicate via the server. 
* I will also write a cutsom nodejs framework to handle communication to the database. 
* Might need to add support for more languages later on but we will start with nodejs implementation. 


#### V9 for work.
V9 Database will be used in a VR project.
for commnunication with the VR and computer/Crane we can add socket transport. 
So the data from the crane will send a socket communction via TCP with the postion data
this will make it easy to send data and we dont need to think about read file latency.

also for the porject we can use Noa Test Tool to simulate common crane movment. for showcasing if we dont have a crane to show.
we can also try new things that we dont have time to program a phyical crane with the test tool.

for the VR crane I want it so it can predict how it will move. So we are going to change the test files to also contain the end position of the crane. and make the VR crane move abit faster so it allways is done before the phyical crane therefor making it easy to sync the machines.
