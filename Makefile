run_00:
	cd 00_smoke_test && go run echo_862.go &

kill_00:
	pkill -f echo_862.go || true

