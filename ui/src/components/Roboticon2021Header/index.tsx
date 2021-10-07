import styles from "./index.module.scss"

export default function Roboticon2021Header() {

    return (
        <header className={styles.header}>
            <div className={styles.mainHeader}>
                <div className={styles.mainHeaderAction}>
                    <picture>
                        <source srcSet="/img/eao_bird_circle.webp" type="image/webp" />
                        <source srcSet="/img/eao_bird_circle.png" type="image/png" />
                        <img className={styles.headerLogo} src="/img/eao_bird_circle.png" alt="Edgar Allan Ohms Logo" />
                    </picture>
                    <span className={styles.headerText}>
                        Nevermore FMS
                        &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
                        |
                        &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
                        ROBOTICON 2021</span>
                    <picture>
                        <img className={styles.headerLogo} src="/img/roboticon-logo.png" alt="Roboticon Logo" />
                    </picture>
                </div>
            </div>
        </header >
    )
}