import test from 'ava'

import { compareVersions } from '../index.js'

test('sum from native', (t) => {
  const result = compareVersions('1.0.0', '1.0.1')
  t.is(result, -1)
})
