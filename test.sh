set -x
set -eo pipefail

if [[ -z $1 ]] 
then 
  echo "No argument supplied, please supply in format d<number>q<number>"
  exit 1
fi

cargo watch --ignore output --ignore question -x "test --bin $1"
