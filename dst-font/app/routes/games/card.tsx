import { useTranslation } from "react-i18next";
import classes from './card.module.css'



export default function GameCard(
  props: {
    title: string,
    game_mod?: string | null,
    description: string,
    password: string,
    player: string,
    status: string,
  }
) {
  const { title, game_mod, description, password, player, status } = props;

  const { t } = useTranslation();


  return (
    <div className="card">
      <header className="card-header">
        <p className="card-header-title">{title}</p>
      </header>
      <div className="card-content">
        <div className="content">
          <div className={classes['cus-item']}>
            <div>{t('card.status')}</div>
            <div>{status}</div>
          </div>
          <div className={classes['cus-item']}>
            <div>{t('card.game_mod')}</div>
            <div>{t('card.hybrid')}</div>
          </div>
          <div className={classes['cus-item']}>
            <div>{t('card.description')}</div>
            <div>{description}</div>
          </div>
          <div className={classes['cus-item']}>
            <div>{t('card.password')}</div>
            <div>{password}</div>
          </div>
          <div className={classes['cus-item']}>
            <div>{t('card.player')}</div>
            <div>{player}</div>
          </div>
        </div>
      </div>
      <footer className="card-footer">
        <a href="#" className="card-footer-item">{t('card.start')}</a>
        <a href="#" className="card-footer-item">{t('card.backup')}</a>
        <a href="#" className="card-footer-item">{t('card.download')}</a>
      </footer>
    </div>
  );
}