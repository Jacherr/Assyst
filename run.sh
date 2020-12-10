#! /bin/sh

while :
do
        echo "Starting process..."
	cd /home/assyst/dist/src
        node run
        if [ $? -eq 0 ]
        then
                echo "Stopping process..." # exit shell script if exit code is 0
                break
        fi
        sleep 10 # wait 10 seconds before restarting
done
