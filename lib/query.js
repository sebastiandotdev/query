import child_process from 'node:child_process'
import path from 'node:path'
import fs from 'node:fs'
import os from 'node:os'



const repo_dir = path.dirname(__dirname)
const npm_dir = path.join(repo_dir, 'npm', 'query')

/**
 * 
 * @param {string} query_path query transpiled file path
 */
const build_lib = (query_path) => {
    const lib_dir = path.join(npm_dir, 'lib')
}