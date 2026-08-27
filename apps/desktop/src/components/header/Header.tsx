import React from 'react';
import { useTranslation } from 'react-i18next';
import { Settings, X, Minus, Square } from 'lucide-react';
import { SUPPORTED_UI_LANGUAGES } from '@dubly/shared';
import { useSettingsStore } from '../../stores/settingsStore';
import MascotImage from '../../assets/mascot.png';
import { getCurrentWindow } from '@tauri-apps/api/window';

interface HeaderProps {
  onOpenSettings: () => void;
}

/** Ghost icon button used for all header controls - one quiet visual voice. */
const HeaderButton: React.FC<{
  onClick: () => void;
  label: string;
  danger?: boolean;
  children: React.ReactNode;
}> = ({ onClick, label, danger = false, children }) => (
  <button
    onClick={onClick}
    title={label}
    aria-label={label}
    className={`w-8 h-8 rounded-lg flex items-center justify-center transition-colors cursor-pointer text-base-content/50 ${
      danger
        ? 'hover:bg-error/15 hover:text-error'
        : 'hover:bg-base-content/5 hover:text-base-content'
    }`}
  >
    {children}
  </button>
);

export const Header: React.FC<HeaderProps> = ({ onOpenSettings }) => {
  const { t, i18n } = useTranslation();
  const { updateSettings } = useSettingsStore();

  const toggleLanguage = () => {
    const nextLang = i18n.language === 'en' ? 'fa' : 'en';
    updateSettings({ uiLanguage: nextLang });
  };

  const currentLangObj = SUPPORTED_UI_LANGUAGES.find((l) => l.code === i18n.language);
  const appWindow = getCurrentWindow();

  return (
    <header
      data-tauri-drag-region
      className="w-full h-14 px-4 flex items-center justify-between select-none shrink-0 border-b border-base-content/5"
    >
      <div data-tauri-drag-region className="flex items-center gap-2.5 pointer-events-none">
        <img src={MascotImage} alt="" className="w-7 h-7 object-contain" />
        <span className="text-sm font-semibold text-base-content tracking-tight">
          {t('common.appName')}
        </span>
        <span className="text-[11px] text-base-content/40 font-medium hidden sm:block">
          {t('common.tagline')}
        </span>
      </div>

      <div className="flex items-center gap-1">
        <button
          onClick={toggleLanguage}
          title={t('settings.appLanguage')}
          className="h-8 px-2.5 rounded-lg text-[11px] font-semibold uppercase flex items-center gap-1.5 transition-colors cursor-pointer text-base-content/50 hover:bg-base-content/5 hover:text-base-content"
        >
          {currentLangObj?.countryCode ? (
            <span className={`fi fi-${currentLangObj.countryCode} text-xs rounded-[3px]`} />
          ) : (
            <span>{currentLangObj?.flag}</span>
          )}
          <span>{currentLangObj?.code}</span>
        </button>

        <HeaderButton onClick={onOpenSettings} label={t('settings.title')}>
          <Settings className="w-4 h-4" />
        </HeaderButton>

        <div className="h-4 w-px bg-base-content/10 mx-1.5 shrink-0" />

        <HeaderButton onClick={() => appWindow.minimize()} label="Minimize">
          <Minus className="w-4 h-4" />
        </HeaderButton>

        <HeaderButton onClick={() => appWindow.toggleMaximize()} label="Maximize">
          <Square className="w-3.5 h-3.5" />
        </HeaderButton>

        <HeaderButton onClick={() => appWindow.close()} label="Close" danger>
          <X className="w-4 h-4" />
        </HeaderButton>
      </div>
    </header>
  );
};
