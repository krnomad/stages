입력된 git 정보를 바탕으로 repo manifest을 통해 checkout된 소스에 local merge를 수행하는 파이프라인을 stage CLI

## local merge tool
- google repo tool을 사용해 checkout된 소스에 대해 local merge를 수행하는 도구
- merge 정책을 설정해 merge를 수행할 수 있음 (fast-forward, rebase, merge)

## usage
- repo-manager -i pr.json -p ff : pr.json에 정의된 git 정보를 바탕으로 repo manifest를 통해 checkout된 소스에 대해 local merge를 수행
  - repo-manager --input pr.json --policy ff