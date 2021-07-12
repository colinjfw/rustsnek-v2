#!/bin/bash

ssh -i ./build/deploy.pem ec2-user@54.202.154.84 'sudo systemctl stop snake.service'
scp -i ./build/deploy.pem ./build/snake ec2-user@54.202.154.84:/home/ec2-user/snake
ssh -i ./build/deploy.pem ec2-user@54.202.154.84 'sudo systemctl start snake.service'
