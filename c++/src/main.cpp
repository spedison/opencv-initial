#include "video_main.hpp" 
#include "rotaciona_main.hpp" 


int main (int argc, char** argv){
 
   if (argv[1][0] == 'v' || argv[1][0] == 'V')
	   video_main(argc, argv);
   else if (argv[1][0] == 'r' || argv[1][0] == 'R')
	   rotaciona_main(argc, argv);

}
