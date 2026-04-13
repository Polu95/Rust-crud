export default function UserItem({ user, onEdit, onDelete }) {
  return (
    <li className="item">
      <div>
        <strong>{user.name}</strong>
        <span className="email">{user.email}</span>
      </div>
      <div className="actions">
        <button onClick={() => onEdit(user)}>Edit</button>
        <button onClick={() => onDelete(user)} className="danger">
          Delete
        </button>
      </div>
    </li>
  )
}
