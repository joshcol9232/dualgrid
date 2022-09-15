#ifndef INTERSECTION_H
#define INTERSECTION_H

#include "vector.h"

template<size_t N>
class Intersection
{
public:
    Vector<N> r;
    size_t j1;      // Parent sets
    size_t j2;

    Intersection(Vector<N> r_, size_t j1_, size_t j2_): r(r_), j1(j1_), j2(j2_) {}
};

#endif // INTERSECTION_H
