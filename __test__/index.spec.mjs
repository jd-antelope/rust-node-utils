import test from 'ava'

import { compareVersions } from '../index.js'

test('sum from native', (t) => {
  t.is(compareVersions('1.3.1', '1.3.2'), 3)
})
