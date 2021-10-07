import styles from "./idle.module.scss"

export default function IdleDisplay() {

    return (
        <div className={styles.layer1}>
            <div className={styles.layer2} />
            <div className={styles.content}>
                <img src="/img/roboticon-logo-with-name.png" alt="" />
            </div>
        </div>
    )
}