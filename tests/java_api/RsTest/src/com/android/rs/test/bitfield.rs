#include "shared.rsh"

// There is a C99 rule (under "Structure and union members") that
// reads "One special guarantee is made in order to simplify the use
// of unions: if a union contains several structures that share a
// common initial sequence, and if the union object currently contains
// one of these structures, it is permitted to inspect the common
// initial part of any of them anywhere that a declaration of the
// completed type of the union is visible. Two structures share a
// common initial sequence if corresponding members have compatible
// types (and, for bit-fields, the same widths) for a sequence of one
// or more initial members."
//
// We want to ensure that the common initial sequences of exported
// and non-exported types have the same layout.

// An exported type (because we declare a global variable of this type)
struct NoBitfield {
    int I;
    // expect 4 bytes of padding here
    long L;
    float F;
    // expect 4 bytes of padding here
};

struct NoBitfield junk;  // just to make this an exported type

// A non-exported type that shares a common initial sequence with NoBitfield
struct Bitfield {
    int I;
    // expect 4 bytes of padding here
    long L;
    uint U:3;
};

union CommonInitialSequence {
    struct NoBitfield nbf;
    struct   Bitfield  bf;
};

static union CommonInitialSequence U, V;

static struct NoBitfield *nbf;
static struct   Bitfield * bf;

// Note: Sets through the exported type (NoBitfield)
void setUnion(long argL, int argI) {
    nbf->L = argL;
    nbf->I = argI;
}

// Note: Tests through the non-exported type (Bitfield)
void testUnion(long argL, int argI) {
    bool failed = false;

    rsDebug("argI    ", argI);
    rsDebug("bf->I   ", bf->I);
    rsDebug("argL.lo ", (unsigned)argL & ~0U);
    rsDebug("bf->L.lo", (unsigned)bf->L & ~0U);
    rsDebug("argL.hi ", (unsigned)((ulong)argL >> 32));
    rsDebug("bf->L.hi", (unsigned)((ulong)bf->L >> 32));

    _RS_ASSERT(bf->I == argI);
    _RS_ASSERT(bf->L == argL);

    if (failed) {
        rsDebug("bitfield FAILED", 0);
        rsSendToClientBlocking(RS_MSG_TEST_FAILED);
    }
    else {
        rsDebug("bitfield PASSED", 0);
        rsSendToClientBlocking(RS_MSG_TEST_PASSED);
    }
}

// Note: Prevent compiler from optimizing setUnion()/testUnion()
//       to convert indirect accesses through nbf/bf into direct
//       accesses through U or V.
void choose(int i) {
    if (i) {
        nbf = &U.nbf;
         bf = &U. bf;
    } else {
        nbf = &V.nbf;
         bf = &V. bf;
    }
}
