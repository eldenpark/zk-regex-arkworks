pragma circom 2.1.5;

include "@zk-email/zk-regex-circom/circuits/regex_helpers.circom";

// regex: a
template SimpleRegex(msg_bytes) {
	signal input msg[msg_bytes];
	signal output out;

	var num_bytes = msg_bytes+1;
	signal in[num_bytes];
	in[0]<==255;
	for (var i = 0; i < msg_bytes; i++) {
		in[i+1] <== msg[i];
	}

	component eq[1][num_bytes];
	component and[1][num_bytes];
	signal states[num_bytes+1][2];
	signal states_tmp[num_bytes+1][2];
	signal from_zero_enabled[num_bytes+1];
	from_zero_enabled[num_bytes] <== 0;
	component state_changed[num_bytes];

	for (var i = 1; i < 2; i++) {
		states[0][i] <== 0;
	}

	for (var i = 0; i < num_bytes; i++) {
		state_changed[i] = MultiOR(1);
		states[i][0] <== 1;
		eq[0][i] = IsEqual();
		eq[0][i].in[0] <== in[i];
		eq[0][i].in[1] <== 97;
		and[0][i] = AND();
		and[0][i].a <== states[i][0];
		and[0][i].b <== eq[0][i].out;
		states_tmp[i+1][1] <== 0;
		from_zero_enabled[i] <== MultiNOR(1)([states_tmp[i+1][1]]);
		states[i+1][1] <== MultiOR(2)([states_tmp[i+1][1], from_zero_enabled[i] * and[0][i].out]);
		state_changed[i].in[0] <== states[i+1][1];
	}

	component is_accepted = MultiOR(num_bytes+1);
	for (var i = 0; i <= num_bytes; i++) {
		is_accepted.in[i] <== states[i][1];
	}
	out <== is_accepted.out;
	signal is_consecutive[msg_bytes+1][3];
	is_consecutive[msg_bytes][2] <== 0;
	for (var i = 0; i < msg_bytes; i++) {
		is_consecutive[msg_bytes-1-i][0] <== states[num_bytes-i][1] * (1 - is_consecutive[msg_bytes-i][2]) + is_consecutive[msg_bytes-i][2];
		is_consecutive[msg_bytes-1-i][1] <== state_changed[msg_bytes-i].out * is_consecutive[msg_bytes-1-i][0];
		is_consecutive[msg_bytes-1-i][2] <== ORAnd()([(1 - from_zero_enabled[msg_bytes-i+1]), states[num_bytes-i][1], is_consecutive[msg_bytes-1-i][1]]);
	}
	// substrings calculated: [{(2, 3)}, {(6, 7), (7, 7)}, {(8, 9)}]
	signal prev_states0[1][msg_bytes];
	signal is_substr0[msg_bytes];
	signal is_reveal0[msg_bytes];
	signal output reveal0[msg_bytes];
	for (var i = 0; i < msg_bytes; i++) {
		 // the 0-th substring transitions: [(2, 3)]
		prev_states0[0][i] <== (1 - from_zero_enabled[i+1]) * states[i+1][2];
		is_substr0[i] <== MultiOR(1)([prev_states0[0][i] * states[i+2][3]]);
		is_reveal0[i] <== MultiAND(3)([out, is_substr0[i], is_consecutive[i][2]]);
		reveal0[i] <== in[i+1] * is_reveal0[i];
	}
	signal prev_states1[2][msg_bytes];
	signal is_substr1[msg_bytes];
	signal is_reveal1[msg_bytes];
	signal output reveal1[msg_bytes];
	for (var i = 0; i < msg_bytes; i++) {
		 // the 1-th substring transitions: [(6, 7), (7, 7)]
		prev_states1[0][i] <== (1 - from_zero_enabled[i+1]) * states[i+1][6];
		prev_states1[1][i] <== (1 - from_zero_enabled[i+1]) * states[i+1][7];
		is_substr1[i] <== MultiOR(2)([prev_states1[0][i] * states[i+2][7], prev_states1[1][i] * states[i+2][7]]);
		is_reveal1[i] <== MultiAND(3)([out, is_substr1[i], is_consecutive[i][2]]);
		reveal1[i] <== in[i+1] * is_reveal1[i];
	}
	signal prev_states2[1][msg_bytes];
	signal is_substr2[msg_bytes];
	signal is_reveal2[msg_bytes];
	signal output reveal2[msg_bytes];
	for (var i = 0; i < msg_bytes; i++) {
		 // the 2-th substring transitions: [(8, 9)]
		prev_states2[0][i] <== (1 - from_zero_enabled[i+1]) * states[i+1][8];
		is_substr2[i] <== MultiOR(1)([prev_states2[0][i] * states[i+2][9]]);
		is_reveal2[i] <== MultiAND(3)([out, is_substr2[i], is_consecutive[i][2]]);
		reveal2[i] <== in[i+1] * is_reveal2[i];
	}
}