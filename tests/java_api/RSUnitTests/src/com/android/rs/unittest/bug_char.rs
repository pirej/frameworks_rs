/*
 * Copyright (C) 2017 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#include "shared.rsh"

char rand_sc1_0, rand_sc1_1;
char2 rand_sc2_0, rand_sc2_1;

char min_rand_sc1_sc1;
char2 min_rand_sc2_sc2;

static bool test_bug_char() {
    bool failed = false;

    rsDebug("rand_sc2_0.x: ", rand_sc2_0.x);
    rsDebug("rand_sc2_0.y: ", rand_sc2_0.y);
    rsDebug("rand_sc2_1.x: ", rand_sc2_1.x);
    rsDebug("rand_sc2_1.y: ", rand_sc2_1.y);
    char temp_sc1;
    char2 temp_sc2;

    temp_sc1 = min( rand_sc1_0, rand_sc1_1 );
    if (temp_sc1 != min_rand_sc1_sc1) {
        rsDebug("temp_sc1", temp_sc1);
        failed = true;
    }
    rsDebug("broken", 'y');

    temp_sc2 = min( rand_sc2_0, rand_sc2_1 );
    if (temp_sc2.x != min_rand_sc2_sc2.x
            || temp_sc2.y != min_rand_sc2_sc2.y) {
        failed = true;
    }


    return failed;
}

void bug_char_test() {
    bool failed = false;
    failed |= test_bug_char();

    if (failed) {
        rsSendToClientBlocking(RS_MSG_TEST_FAILED);
    }
    else {
        rsSendToClientBlocking(RS_MSG_TEST_PASSED);
    }
}

