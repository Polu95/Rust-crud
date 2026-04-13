import { useCallback, useEffect, useState } from 'react'
import * as api from '../services/userApi'

export function useUsers() {
  const [users, setUsers] = useState([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState('')

  const load = useCallback(async () => {
    setLoading(true)
    setError('')
    try {
      setUsers(await api.fetchUsers())
    } catch (e) {
      setError(e.message)
    } finally {
      setLoading(false)
    }
  }, [])

  useEffect(() => { load() }, [load])

  const create = async (user) => { await api.createUser(user); await load() }
  const update = async (id, user) => { await api.updateUser(id, user); await load() }
  const remove = async (id) => { await api.deleteUser(id); await load() }

  return { users, loading, error, reload: load, create, update, remove }
}
