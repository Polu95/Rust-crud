import UserItem from './UserItem'
import { getUserId } from '../services/userApi'

export default function UserList({ users, onEdit, onDelete }) {
  if (users.length === 0) {
    return <p className="empty">No users yet</p>
  }

  return (
    <ul className="list">
      {users.map((u) => (
        <UserItem
          key={getUserId(u)}
          user={u}
          onEdit={onEdit}
          onDelete={onDelete}
        />
      ))}
    </ul>
  )
}
