import { useState } from 'react'
import UserForm from './components/UserForm'
import UserList from './components/UserList'
import { useUsers } from './hooks/useUsers'
import { getUserId } from './services/userApi'

export default function App() {
  const { users, loading, error, create, update, remove } = useUsers()
  const [editing, setEditing] = useState(null)

  const handleSubmit = async (data) => {
    if (editing) {
      await update(getUserId(editing), data)
      setEditing(null)
    } else {
      await create(data)
    }
  }

  const handleDelete = async (u) => {
    if (!confirm(`Delete ${u.name}?`)) return
    await remove(getUserId(u))
  }

  return (
    <div className="container">
      <h1>Rust CRUD Users</h1>

      <UserForm
        editing={editing}
        onSubmit={handleSubmit}
        onCancel={() => setEditing(null)}
      />

      {error && <p className="error">{error}</p>}
      {loading ? (
        <p>Loading...</p>
      ) : (
        <UserList users={users} onEdit={setEditing} onDelete={handleDelete} />
      )}
    </div>
  )
}
