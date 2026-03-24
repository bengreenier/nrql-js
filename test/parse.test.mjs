import test from 'node:test'
import assert from 'node:assert/strict'
import { parseNrql } from '../index.js'

test('parseNrql minimal FROM … SELECT', () => {
  const q = parseNrql('FROM Transaction SELECT *')
  assert.equal(q.from.eventTypes.length, 1)
  assert.equal(q.from.eventTypes[0], 'Transaction')
  assert.equal(q.select.items.length, 1)
  assert.equal(q.select.items[0].type, 'Wildcard')
})

test('parseNrql throws on invalid input', () => {
  assert.throws(() => parseNrql('not nrql at all'), /parse error/)
})
