import './Header.css';

const Header = () => {
    return (
        <header className="header">
            <div className="header-container">
                <div className="logo">
                    <h1>Rust Web Template</h1>
                </div>
                <nav className="navigation">
                    <ul className="nav-list">
                        <li className="nav-item">
                            <a href="#home" className="nav-link">ホーム</a>
                        </li>
                        <li className="nav-item">
                            <a href="#about" className="nav-link">概要</a>
                        </li>
                        <li className="nav-item">
                            <a href="#services" className="nav-link">サービス</a>
                        </li>
                    </ul>
                </nav>
            </div>
        </header>
    );
};

export default Header;
