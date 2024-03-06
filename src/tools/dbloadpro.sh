#!/bin/bash

while getopts ':d::c:l::n:' OPT &> /dev/null;do 
 case $OPT in 
 d) 
 dbname=$OPTARG ;; 
 c) 
 config=$OPTARG ;; 
 l)
 log=$OPTARG ;;
 n)
 nrows=$OPTARG ;;
 *) 
 echo "Wrong Options" 
 exit 7 ;; 
 esac 
done 