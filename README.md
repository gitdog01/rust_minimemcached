# rust_minimemcached

rust를 공부하면서 ,해당 레포 클론해보기 나만의 작은 memory cache 만들어보기

현재 구현된 기능

- get <key> : 키를 기반으로 가져옵니다. 값이 없으면 None 을 표시합니다.
- set <key> <value>: 키를 기반으로 int value 를 저장합니다. 다른 값은 아직 지원 안합니다.
