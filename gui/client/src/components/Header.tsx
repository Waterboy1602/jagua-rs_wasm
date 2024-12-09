import { Link } from "react-router-dom"; // Optional, if using React Router for navigation
import styles from "../style/Header.module.css"; // Optional CSS file for styling

const Header = () => {
    return (
        <header className={styles.header}>
            <Link to="/" className={styles.header}>
                <img src="./jaguars_logo.svg" alt="jagua-rs logo" />
                <h1>jagua-rs</h1>
            </Link>
        </header>
    );
};

export default Header;
