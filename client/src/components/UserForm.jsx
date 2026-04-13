import { useEffect, useState } from 'react'

export default function UserForm({ editing, onSubmit, onCancel }) {
  const [name, setName] = useState('')
  const [email, setEmail] = useState('')

  useEffect(() => {
    setName(editing?.name || '')
    setEmail(editing?.email || '')
  }, [editing])

  const handleSubmit = (e) => {
    e.preventDefault()
    if (!name || !email) return
    onSubmit({ name, email })
    if (!editing) { setName(''); setEmail('') }
  }

  return (
    <form onSubmit={handleSubmit} className="form">
      <input
        placeholder="Name"
        value={name}
        onChange={(e) => setName(e.target.value)}
      />
      <input
        placeholder="Email"
        type="email"
        value={email}
        onChange={(e) => setEmail(e.target.value)}
      />
      <button type="submit">{editing ? 'Update' : 'Create'}</button>
      {editing && (
        <button type="button" onClick={onCancel} className="secondary">
          Cancel
        </button>
      )}
    </form>
  )
}
